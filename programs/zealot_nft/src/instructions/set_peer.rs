use crate::*;

#[derive(Accounts)]
#[instruction(params: SetPeerParams)]
pub struct SetPeer<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,
    #[account(
        init_if_needed,
        payer = admin,
        space = 8 + Peer::INIT_SPACE,
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
    pub system_program: Program<'info, System>,
}

impl SetPeer<'_> {
    pub fn apply(ctx: &mut Context<SetPeer>, params: &SetPeerParams) -> Result<()> {
        ctx.accounts.peer.address = params.peer;
        ctx.accounts.peer.bump = ctx.bumps.peer;
        Ok(())
    }
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct SetPeerParams {
    pub dst_eid: u32,
    pub peer: [u8; 32],
}
