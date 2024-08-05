use crate::*;

#[event]
pub struct ONftSent {
    pub guid: [u8; 32],
    pub dst_eid: u32,
    pub from: Pubkey,
    pub amount_sent_ld: u64,
    pub amount_received_ld: u64,
}

#[event]
pub struct ONftReceived {
    pub guid: [u8; 32],
    pub src_eid: u32,
    pub to: Pubkey,
    pub amount_received_ld: u64,
}
