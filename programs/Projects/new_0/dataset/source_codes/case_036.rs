use anchor_lang::prelude::*;
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjfA22mvTWf");

#[program]
pub mod rotating_logger_003 {
    use super::*;

    pub fn save_value(ctx: Context<Ctx003>, input: u64) -> Result<()> {
        let index = ctx.accounts.storage.counter % 3;

        let is0 = (index == 0) as u64;
        let is1 = (index == 1) as u64;
        let is2 = (index == 2) as u64;

        let s = &mut ctx.accounts.storage;
        s.value0 = is0 * input + (1 - is0) * s.value0;
        s.value1 = is1 * input + (1 - is1) * s.value1;
        s.value2 = is2 * input + (1 - is2) * s.value2;

        s.counter = s.counter + 1;

        Ok(())
    }

    pub fn display(ctx: Context<Ctx003>) -> Result<()> {
        let s = &ctx.accounts.storage;
        msg!("Slot 0: {}", s.value0);
        msg!("Slot 1: {}", s.value1);
        msg!("Slot 2: {}", s.value2);
        msg!("Counter: {}", s.counter);
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
    pub counter: u64,
    pub value0: u64,
    pub value1: u64,
    pub value2: u64,
}
