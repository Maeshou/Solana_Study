use anchor_lang::prelude::*;
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjfA20mvTWf");

#[program]
pub mod min_value_keeper_003 {
    use super::*;

    pub fn store_minimum(ctx: Context<Ctx003>, candidate: u64) -> Result<()> {
        let current = ctx.accounts.storage.value;
        let smaller = (candidate < current) as u64;
        let larger = 1 - smaller;

        let result = smaller * candidate + larger * current;
        ctx.accounts.storage.value = result;

        Ok(())
    }

    pub fn read(ctx: Context<Ctx003>) -> Result<()> {
        msg!("Current minimum value: {}", ctx.accounts.storage.value);
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
    pub value: u64,
}
