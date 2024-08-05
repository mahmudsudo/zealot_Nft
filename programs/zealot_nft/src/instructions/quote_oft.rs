use crate::*;

#[derive(Accounts)]
#[instruction(params: QuoteONftParams)]
pub struct QuoteONft<'info> {
    #[account(
        seeds = [ONft_SEED, &get_ONft_config_seed(&ONft_config).to_bytes()],
        bump = ONft_config.bump
    )]
    pub ONft_config: Account<'info, ONftConfig>,
    #[account(
        seeds = [
            PEER_SEED,
            &ONft_config.key().to_bytes(),
            &params.dst_eid.to_be_bytes()
        ],
        bump = peer.bump
    )]
    pub peer: Account<'info, Peer>,
    #[account(
        address = ONft_config.token_mint,
    )]
    pub token_mint: InterfaceAccount<'info, anchor_spl::token_interface::Mint>,
}

impl QuoteONft<'_> {
    pub fn apply(ctx: &Context<QuoteONft>, params: &QuoteONftParams) -> Result<QuoteONftResult> {
        // 1. Quote the amount with token2022 fee and dedust it
        let amount_received_ld = ctx.accounts.ONft_config.remove_dust(get_post_fee_amount_ld(
            &ctx.accounts.ONft_config.ext,
            &ctx.accounts.token_mint,
            params.amount_ld,
        )?);
        require!(amount_received_ld >= params.min_amount_ld, ONftError::SlippageExceeded);

        // amount_sent_ld does not have to be dedusted
        let amount_sent_ld = get_pre_fee_amount_ld(
            &ctx.accounts.ONft_config.ext,
            &ctx.accounts.token_mint,
            amount_received_ld,
        )?;
        let ONft_limits = ONftLimits { min_amount_ld: 0, max_amount_ld: 0xffffffffffffffff };
        let ONft_fee_details = if amount_received_ld < amount_sent_ld {
            vec![ONftFeeDetail {
                fee_amount_ld: amount_sent_ld - amount_received_ld,
                description: "Token2022 Transfer Fee".to_string(),
            }]
        } else {
            vec![]
        };
        let ONft_receipt = ONftReceipt { amount_sent_ld, amount_received_ld };
        Ok(QuoteONftResult { ONft_limits, ONft_fee_details, ONft_receipt })
    }
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct QuoteONftParams {
    pub dst_eid: u32,
    pub to: [u8; 32],
    pub amount_ld: u64,
    pub min_amount_ld: u64,
    pub options: Vec<u8>,
    pub compose_msg: Option<Vec<u8>>,
    pub pay_in_lz_token: bool,
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct QuoteONftResult {
    pub ONft_limits: ONftLimits,
    pub ONft_fee_details: Vec<ONftFeeDetail>,
    pub ONft_receipt: ONftReceipt,
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct ONftFeeDetail {
    pub fee_amount_ld: u64,
    pub description: String,
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct ONftReceipt {
    pub amount_sent_ld: u64,
    pub amount_received_ld: u64,
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct ONftLimits {
    pub min_amount_ld: u64,
    pub max_amount_ld: u64,
}
