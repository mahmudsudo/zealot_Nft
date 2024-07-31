use anchor_lang::prelude::*;

declare_id!("BtWXBRL8n79mjRQhY9S4j1FpXJp8UJvcRaT3pZnfquxo");

#[program]
pub mod zealot_nft {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
