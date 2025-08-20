use anchor_lang::prelude::*;
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjfA17mvTWf");

#[program]
pub mod limited_register_003 {
    use super::*;

    pub fn write_within_limit(ctx: Context<Ctx003>, input: u64) -> Result<()> {
        let current_limit = ctx.accounts.storage.limit;
        let allow = (input <= current_limit) as u64;
        let new_value = allow * input + (1 - allow) * ctx.accounts.storage.data;
        ctx.accounts.storage.data = new_value;
        Ok(())
    }

    pub fn read(ctx: Context<Ctx003>) -> Result<()> {
        msg!("Stored: {}", ctx.accounts.storage.data);
        msg!("Limit : {}", ctx.accounts.storage.limit);
        Ok(())
    }

    pub fn set_limit(ctx: Context<Ctx003>, new_limit: u64) -> Result<()> {
        ctx.accounts.storage.limit = new_limit;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ctx003<'info> {
    #[account(mut, has_one = authority)]
    pub storage: Account<'info, Storage003>,
    #[account(signer)]
    pub authority: Signer<'info>,
}

#[account]
pub struct Storage003 {
    pub authority: Pubkey,
    pub data: u64,
    pub limit: u64,
}
