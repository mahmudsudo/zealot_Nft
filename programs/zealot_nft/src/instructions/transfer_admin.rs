use crate::*;

#[derive(Accounts)]
pub struct TransferAdmin<'info> {
    pub admin: Signer<'info>,
    #[account(
        mut,
        seeds = [ONft_SEED, &get_ONft_config_seed(&ONft_config).to_bytes()],
        bump = ONft_config.bump,
        has_one = admin @ONftError::Unauthorized
    )]
    pub ONft_config: Account<'info, ONftConfig>,
}

impl TransferAdmin<'_> {
    pub fn apply(ctx: &mut Context<TransferAdmin>, params: &TransferAdminParams) -> Result<()> {
        ctx.accounts.ONft_config.admin = params.admin;
        Ok(())
    }
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct TransferAdminParams {
    pub admin: Pubkey,
}
