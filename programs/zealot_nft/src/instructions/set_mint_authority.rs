use crate::*;

#[derive(Accounts)]
pub struct SetMintAuthority<'info> {
    /// The admin or the mint authority
    pub signer: Signer<'info>,
    #[account(
        mut,
        seeds = [ONft_SEED, ONft_config.token_mint.as_ref()],
        bump = ONft_config.bump,
        constraint = is_valid_signer(signer.key(), &ONft_config) @ONftError::Unauthorized
    )]
    pub ONft_config: Account<'info, ONftConfig>,
}

impl SetMintAuthority<'_> {
    pub fn apply(
        ctx: &mut Context<SetMintAuthority>,
        params: &SetMintAuthorityParams,
    ) -> Result<()> {
        // the mint authority can be removed by setting it to None
        ctx.accounts.ONft_config.ext = ONftConfigExt::Native(params.mint_authority);
        Ok(())
    }
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct SetMintAuthorityParams {
    pub mint_authority: Option<Pubkey>,
}

/// Check if the signer is the admin or the mint authority
/// When the mint authority is set, the signer can be the admin or the mint authority
/// Otherwise, no one can set the mint authority
fn is_valid_signer(signer: Pubkey, ONft_config: &ONftConfig) -> bool {
    if let ONftConfigExt::Native(Some(mint_authority)) = ONft_config.ext {
        signer == ONft_config.admin || signer == mint_authority
    } else {
        false
    }
}
