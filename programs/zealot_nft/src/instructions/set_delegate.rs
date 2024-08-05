use crate::*;
use endpoint::instructions::SetDelegateParams as EndpointSetDelegateParams;
use oapp::endpoint;

#[derive(Accounts)]
#[instruction(params: SetDelegateParams)]
pub struct SetDelegate<'info> {
    pub admin: Signer<'info>,
    #[account(
        seeds = [ONft_SEED, &get_ONft_config_seed(&ONft_config).to_bytes()],
        bump = ONft_config.bump,
        has_one = admin @ONftError::Unauthorized
    )]
    pub ONft_config: Account<'info, ONftConfig>,
}

impl SetDelegate<'_> {
    pub fn apply(ctx: &mut Context<SetDelegate>, params: &SetDelegateParams) -> Result<()> {
        let ONft_config_seed = get_ONft_config_seed(&ctx.accounts.ONft_config);
        let seeds: &[&[u8]] =
            &[ONft_SEED, &ONft_config_seed.to_bytes(), &[ctx.accounts.ONft_config.bump]];
        let _ = oapp::endpoint_cpi::set_delegate(
            ctx.accounts.ONft_config.endpoint_program,
            ctx.accounts.ONft_config.key(),
            &ctx.remaining_accounts,
            seeds,
            EndpointSetDelegateParams { delegate: params.delegate },
        )?;
        Ok(())
    }
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct SetDelegateParams {
    pub delegate: Pubkey,
}
