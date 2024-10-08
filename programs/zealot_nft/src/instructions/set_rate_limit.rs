use crate::*;

#[derive(Accounts)]
#[instruction(params: SetRateLimitParams)]
pub struct SetRateLimit<'info> {
    pub admin: Signer<'info>,
    #[account(
        mut,
        seeds = [PEER_SEED, &ONft_config.key().to_bytes(), &params.dst_eid.to_be_bytes()],
        bump
    )]
    pub peer: Account<'info, Peer>,
    #[account(
        seeds = [ONft_SEED, &get_ONft_config_seed(&ONft_config).to_bytes()],
        bump = ONft_config.bump,
        has_one = admin @ONftError::Unauthorized
    )]
    pub ONft_config: Account<'info, ONftConfig>,
}

impl SetRateLimit<'_> {
    pub fn apply(ctx: &mut Context<SetRateLimit>, params: &SetRateLimitParams) -> Result<()> {
        if !params.enabled {
            ctx.accounts.peer.rate_limiter = None;
            return Ok(());
        }

        let mut rate_limiter = ctx.accounts.peer.rate_limiter.clone().unwrap_or_default();

        if let Some(capacity) = params.capacity {
            rate_limiter.set_capacity(capacity)?;
        }
        if let Some(refill_rate) = params.refill_per_second {
            rate_limiter.set_rate(refill_rate)?;
        }
        ctx.accounts.peer.rate_limiter = Some(rate_limiter);
        Ok(())
    }
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct SetRateLimitParams {
    pub dst_eid: u32,
    pub refill_per_second: Option<u64>,
    pub capacity: Option<u64>,
    pub enabled: bool,
}
