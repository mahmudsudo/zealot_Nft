use crate::*;
use anchor_spl::token_interface::{Mint, TokenInterface};

/// This instruction should always be in the same transaction as InitializeMint.
/// Otherwise, it is possible for your settings to be front-run by another transaction.
/// If such a case did happen, you should initialize another mint for this ONft.
#[derive(Accounts)]
#[instruction(params: InitONftParams)]
pub struct InitONft<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(
        init,
        payer = payer,
        space = 8 + ONftConfig::INIT_SPACE,
        seeds = [ONft_SEED, token_mint.key().as_ref()],
        bump
    )]
    pub ONft_config: Account<'info, ONftConfig>,
    #[account(
        init,
        payer = payer,
        space = 8 + LzReceiveTypesAccounts::INIT_SPACE,
        seeds = [LZ_RECEIVE_TYPES_SEED, &ONft_config.key().as_ref()],
        bump
    )]
    pub lz_receive_types_accounts: Account<'info, LzReceiveTypesAccounts>,
    #[account(
        mint::authority = ONft_config,
        mint::token_program = token_program
    )]
    pub token_mint: InterfaceAccount<'info, Mint>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

impl InitONft<'_> {
    pub fn apply(ctx: &mut Context<InitONft>, params: &InitONftParams) -> Result<()> {
        ctx.accounts.ONft_config.bump = ctx.bumps.ONft_config;
        ctx.accounts.ONft_config.token_mint = ctx.accounts.token_mint.key();
        ctx.accounts.ONft_config.ext = ONftConfigExt::Native(params.mint_authority);
        ctx.accounts.ONft_config.token_program = ctx.accounts.token_program.key();

        ctx.accounts.lz_receive_types_accounts.ONft_config = ctx.accounts.ONft_config.key();
        ctx.accounts.lz_receive_types_accounts.token_mint = ctx.accounts.token_mint.key();

        let oapp_signer = ctx.accounts.ONft_config.key();
        ctx.accounts.ONft_config.init(
            params.endpoint_program,
            params.admin,
            params.shared_decimals,
            ctx.accounts.token_mint.decimals,
            ctx.remaining_accounts,
            oapp_signer,
        )
    }
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct InitONftParams {
    pub admin: Pubkey,
    pub shared_decimals: u8,
    pub endpoint_program: Option<Pubkey>,
    pub mint_authority: Option<Pubkey>,
}
