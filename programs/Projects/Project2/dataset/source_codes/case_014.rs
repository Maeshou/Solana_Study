
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct FlashLoanCtxvial<'info> {
    #[account(mut)] pub lending_market: Account<'info, DataAccount>,
    #[account(mut)] pub borrower: Account<'info, DataAccount>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAccount {
    pub data: u64,
}

#[program]
pub mod missing_owner_check_014 {
    use super::*;

    pub fn flash_loan(ctx: Context<FlashLoanCtxvial>, amount: u64) -> Result<()> {
        let acct = &mut ctx.accounts.lending_market;
        // custom logic for flash_loan
        for _ in 0..amount { acct.data += 1; }
        msg!("Executed flash_loan logic");
        Ok(())
    }
}
