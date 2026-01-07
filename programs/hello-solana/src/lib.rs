use anchor_lang::prelude::*;

declare_id!("8uRfAPGRs5eNip7jHmgdRuZoXNAyg7hobnq9zeBz34Ck");

#[program]
pub mod hello_solana {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
