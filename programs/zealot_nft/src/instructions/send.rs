use crate::*;
use anchor_spl::token_interface::{
    self, Burn, Mint, TokenAccount, TokenInterface, TransferChecked,
};
use oapp::endpoint::{instructions::SendParams as EndpointSendParams, MessagingReceipt};

#[event_cpi]
#[derive(Accounts)]
#[instruction(params: SendParams)]
pub struct Send<'info> {
    pub signer: Signer<'info>,
    #[account(
        mut,
        seeds = [
            PEER_SEED,
            &ONft_config.key().to_bytes(),
            &params.dst_eid.to_be_bytes()
        ],
        bump = peer.bump
    )]
    pub peer: Account<'info, Peer>,
    #[account(
        seeds = [
            ENFORCED_OPTIONS_SEED,
            &ONft_config.key().to_bytes(),
            &params.dst_eid.to_be_bytes()
        ],
        bump = enforced_options.bump
    )]
    pub enforced_options: Account<'info, EnforcedOptions>,
    #[account(
        seeds = [ONft_SEED, &get_ONft_config_seed(&ONft_config).to_bytes()],
        bump = ONft_config.bump
    )]
    pub ONft_config: Account<'info, ONftConfig>,
    #[account(
        mut,
        token::authority = signer,
        token::mint = token_mint,
        token::token_program = token_program,
    )]
    pub token_source: InterfaceAccount<'info, TokenAccount>,
    #[account(
        mut,
        token::authority = ONft_config.key(),
        token::mint = token_mint,
        token::token_program = token_program,
        constraint = ONft_config.ext == ONftConfigExt::Adapter(token_escrow.key()) @ONftError::InvalidTokenEscrow
    )]
    pub token_escrow: Option<InterfaceAccount<'info, TokenAccount>>,
    #[account(
        mut,
        address = ONft_config.token_mint,
        mint::token_program = token_program
    )]
    pub token_mint: InterfaceAccount<'info, Mint>,
    pub token_program: Interface<'info, TokenInterface>,
}

impl Send<'_> {
    pub fn apply(ctx: &mut Context<Send>, params: &SendParams) -> Result<MessagingReceipt> {
        // 1. Quote the amount with token2022 fee and dedust it
        let amount_received_ld = ctx.accounts.ONft_config.remove_dust(get_post_fee_amount_ld(
            &ctx.accounts.ONft_config.ext,
            &ctx.accounts.token_mint,
            params.amount_ld,
        )?);
        require!(amount_received_ld >= params.min_amount_ld, ONftError::SlippageExceeded);

        // 2. Calculate the (minimum) required amount to send to receive exactly amount_received_ld
        // amount_sent_ld does not have to be dedusted, because it is collected or burned locally
        let amount_sent_ld = get_pre_fee_amount_ld(
            &ctx.accounts.ONft_config.ext,
            &ctx.accounts.token_mint,
            amount_received_ld,
        )?;
        if let Some(rate_limiter) = ctx.accounts.peer.rate_limiter.as_mut() {
            rate_limiter.try_consume(amount_sent_ld)?;
        }
        match &ctx.accounts.ONft_config.ext {
            ONftConfigExt::Adapter(_) => {
                if let Some(escrow_acc) = &mut ctx.accounts.token_escrow {
                    // lock
                    token_interface::transfer_checked(
                        CpiContext::new(
                            ctx.accounts.token_program.to_account_info(),
                            TransferChecked {
                                from: ctx.accounts.token_source.to_account_info(),
                                mint: ctx.accounts.token_mint.to_account_info(),
                                to: escrow_acc.to_account_info(),
                                authority: ctx.accounts.signer.to_account_info(),
                            },
                        ),
                        amount_sent_ld,
                        ctx.accounts.token_mint.decimals,
                    )?;
                } else {
                    return Err(ONftError::InvalidTokenEscrow.into());
                }
            },
            ONftConfigExt::Native(_) => {
                // burn
                let cpi_accounts = Burn {
                    mint: ctx.accounts.token_mint.to_account_info(),
                    from: ctx.accounts.token_source.to_account_info(),
                    authority: ctx.accounts.signer.to_account_info(),
                };
                let cpi_program = ctx.accounts.token_program.to_account_info();
                token_interface::burn(CpiContext::new(cpi_program, cpi_accounts), amount_sent_ld)?;
            },
        };

        require!(
            ctx.accounts.ONft_config.key() == ctx.remaining_accounts[1].key(),
            ONftError::InvalidSender
        );
        let amount_sd = ctx.accounts.ONft_config.ld2sd(amount_received_ld);
        let receipt = oapp::endpoint_cpi::send(
            ctx.accounts.ONft_config.endpoint_program,
            ctx.accounts.ONft_config.key(),
            ctx.remaining_accounts,
            &[
                ONft_SEED,
                &get_ONft_config_seed(&ctx.accounts.ONft_config).to_bytes(),
                &[ctx.accounts.ONft_config.bump],
            ],
            EndpointSendParams {
                dst_eid: params.dst_eid,
                receiver: ctx.accounts.peer.address,
                message: msg_codec::encode(
                    params.to,
                    amount_sd,
                    ctx.accounts.signer.key(),
                    &params.compose_msg,
                ),
                options: ctx
                    .accounts
                    .enforced_options
                    .combine_options(&params.compose_msg, &params.options)?,
                native_fee: params.native_fee,
                lz_token_fee: params.lz_token_fee,
            },
        )?;

        emit_cpi!(ONftSent {
            guid: receipt.guid,
            dst_eid: params.dst_eid,
            from: ctx.accounts.token_source.key(),
            amount_sent_ld,
            amount_received_ld
        });

        Ok(receipt)
    }
}

pub fn get_ONft_config_seed(ONft_config: &ONftConfig) -> Pubkey {
    if let ONftConfigExt::Adapter(token_escrow) = ONft_config.ext {
        token_escrow
    } else {
        ONft_config.token_mint
    }
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct SendParams {
    pub dst_eid: u32,
    pub to: [u8; 32],
    pub amount_ld: u64,
    pub min_amount_ld: u64,
    pub options: Vec<u8>,
    pub compose_msg: Option<Vec<u8>>,
    pub native_fee: u64,
    pub lz_token_fee: u64,
}
