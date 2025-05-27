
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct BorrowCtxupii<'info> {
    #[account(mut)] pub reserve: Account<'info, DataAccount>,
    #[account(mut)] pub borrower: Account<'info, DataAccount>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAccount {
    pub data: u64,
}

#[program]
pub mod missing_owner_check_012 {
    use super::*;

    pub fn borrow(ctx: Context<BorrowCtxupii>, amount: u64) -> Result<()> {
        let acct = &mut ctx.accounts.reserve;
        // custom logic for borrow
        let temp = acct.data; acct.data = temp.checked_mul(2).unwrap();
        msg!("Executed borrow logic");
        Ok(())
    }
}
