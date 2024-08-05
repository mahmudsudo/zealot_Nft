use crate::*;
use anchor_spl::token_interface::{Mint, TokenAccount, TokenInterface};

#[derive(Accounts)]
#[instruction(params: InitAdapterONftParams)]
pub struct InitAdapterONft<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(
        init,
        payer = payer,
        space = 8 + ONftConfig::INIT_SPACE,
        seeds = [ONft_SEED, token_escrow.key().as_ref()],
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
    #[account(mint::token_program = token_program)]
    pub token_mint: InterfaceAccount<'info, Mint>,
    #[account(
        init,
        payer = payer,
        token::authority = ONft_config,
        token::mint = token_mint,
        token::token_program = token_program,
    )]
    pub token_escrow: InterfaceAccount<'info, TokenAccount>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

impl InitAdapterONft<'_> {
    pub fn apply(ctx: &mut Context<InitAdapterONft>, params: &InitAdapterONftParams) -> Result<()> {
        ctx.accounts.ONft_config.bump = ctx.bumps.ONft_config;
        ctx.accounts.ONft_config.token_mint = ctx.accounts.token_mint.key();
        ctx.accounts.ONft_config.ext = ONftConfigExt::Adapter(ctx.accounts.token_escrow.key());
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
pub struct InitAdapterONftParams {
    pub admin: Pubkey,
    pub shared_decimals: u8,
    pub endpoint_program: Option<Pubkey>,
}
