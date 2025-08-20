use anchor_lang::prelude::*;
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjfA18mvTWf");

#[program]
pub mod delta_tracker_003 {
    use super::*;

    pub fn update_value(ctx: Context<Ctx003>, input: u64) -> Result<()> {
        let prev = ctx.accounts.storage.last_value;
        let diff = if input > prev {
            input - prev
        } else {
            prev - input
        };

        ctx.accounts.storage.delta = diff;
        ctx.accounts.storage.last_value = input;
        Ok(())
    }

    pub fn show(ctx: Context<Ctx003>) -> Result<()> {
        let s = &ctx.accounts.storage;
        msg!("Last Value: {}", s.last_value);
        msg!("Delta     : {}", s.delta);
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
    pub last_value: u64,
    pub delta: u64,
}
