use anchor_lang::prelude::*;

declare_id!("8uRfAPGRs5eNip7jHmgdRuZoXNAyg7hobnq9zeBz34Ck");

#[program]
pub mod hello_solana {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>,hello:String) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        let data_account = &mut ctx.accounts.data_account;
        data_account.hello = hello;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
        init,
        payer = signer,
        space =  8 + 4 + 200    
    )]
    pub data_account: Account<'info,Hello>,
    pub system_program: Program<'info,System>,
}


#[account]
pub struct Hello {
    pub hello: String,
}