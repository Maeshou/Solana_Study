
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct DecrementCtxvjdt<'info> {
    #[account(mut)] pub counter: Account<'info, DataAccount>,
    #[account(mut)] pub caller: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAccount {
    pub data: u64,
}

#[program]
pub mod missing_owner_check_061 {
    use super::*;

    pub fn decrement(ctx: Context<DecrementCtxvjdt>, amount: u64) -> Result<()> {
        let acct = &mut ctx.accounts.counter;
        // custom logic for decrement
        **ctx.accounts.counter.to_account_info().try_borrow_mut_lamports()? += amount;
        msg!("Executed decrement logic");
        Ok(())
    }
}
