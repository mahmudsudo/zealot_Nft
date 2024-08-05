use anchor_lang::prelude::*;

pub mod compose_msg_codec;
mod errors;
mod events;
mod instructions;
pub mod msg_codec;
pub mod state;

use errors::*;
use events::*;
use instructions::*;
use oapp::{
    endpoint::{MessagingFee, MessagingReceipt},
    LzReceiveParams,
};
use state::*;

declare_id!("HRPXLCqspQocTjfcX4rvAPaY9q6Gwb1rrD3xXWrfJWdW");

pub const ONft_VERSION: u64 = 1;
pub const ONft_SDK_VERSION: u64 = 1;
pub const ONft_SEED: &[u8] = b"ONft";
pub const PEER_SEED: &[u8] = b"Peer";
pub const ENFORCED_OPTIONS_SEED: &[u8] = b"EnforcedOptions";
pub const LZ_RECEIVE_TYPES_SEED: &[u8] = oapp::LZ_RECEIVE_TYPES_SEED;

#[program]
pub mod ONft {
    use super::*;

    pub fn version(_ctx: Context<GetVersion>) -> Result<Version> {
        Ok(Version {
            sdk_version: ONft_SDK_VERSION,
            ONft_version: ONft_VERSION,
        })
    }

    pub fn init_ONft(mut ctx: Context<InitONft>, params: InitONftParams) -> Result<()> {
        InitONft::apply(&mut ctx, &params)
    }

    pub fn init_adapter_ONft(
        mut ctx: Context<InitAdapterONft>,
        params: InitAdapterONftParams,
    ) -> Result<()> {
        InitAdapterONft::apply(&mut ctx, &params)
    }

    // ============================== Admin ==============================
    pub fn transfer_admin(
        mut ctx: Context<TransferAdmin>,
        params: TransferAdminParams,
    ) -> Result<()> {
        TransferAdmin::apply(&mut ctx, &params)
    }

    pub fn set_peer(mut ctx: Context<SetPeer>, params: SetPeerParams) -> Result<()> {
        SetPeer::apply(&mut ctx, &params)
    }

    pub fn set_enforced_options(
        mut ctx: Context<SetEnforcedOptions>,
        params: SetEnforcedOptionsParams,
    ) -> Result<()> {
        SetEnforcedOptions::apply(&mut ctx, &params)
    }

    pub fn set_mint_authority(
        mut ctx: Context<SetMintAuthority>,
        params: SetMintAuthorityParams,
    ) -> Result<()> {
        SetMintAuthority::apply(&mut ctx, &params)
    }

    pub fn mint_to(mut ctx: Context<MintTo>, params: MintToParams) -> Result<()> {
        MintTo::apply(&mut ctx, &params)
    }

    // ============================== Public ==============================

    pub fn quote_ONft(ctx: Context<QuoteONft>, params: QuoteONftParams) -> Result<QuoteONftResult> {
        QuoteONft::apply(&ctx, &params)
    }

    pub fn quote(ctx: Context<Quote>, params: QuoteParams) -> Result<MessagingFee> {
        Quote::apply(&ctx, &params)
    }

    pub fn send(mut ctx: Context<Send>, params: SendParams) -> Result<MessagingReceipt> {
        Send::apply(&mut ctx, &params)
    }

    pub fn lz_receive(mut ctx: Context<LzReceive>, params: LzReceiveParams) -> Result<()> {
        LzReceive::apply(&mut ctx, &params)
    }

    pub fn lz_receive_types(
        ctx: Context<LzReceiveTypes>,
        params: LzReceiveParams,
    ) -> Result<Vec<oapp::endpoint_cpi::LzAccount>> {
        LzReceiveTypes::apply(&ctx, &params)
    }

    pub fn set_rate_limit(
        mut ctx: Context<SetRateLimit>,
        params: SetRateLimitParams,
    ) -> Result<()> {
        SetRateLimit::apply(&mut ctx, &params)
    }

    // Set the LayerZero endpoint delegate for OApp admin functions
    pub fn set_delegate(mut ctx: Context<SetDelegate>, params: SetDelegateParams) -> Result<()> {
        SetDelegate::apply(&mut ctx, &params)
    }
}

#[derive(Accounts)]
pub struct GetVersion {}

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct Version {
    pub sdk_version: u64,
    pub ONft_version: u64,
}
