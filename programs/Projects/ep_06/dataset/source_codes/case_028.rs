use anchor_lang::prelude::*;
declare_id!("REPA0281111111111111111111111111111111111111");

#[program]
pub mod case028 {
    use super::*;
    pub fn execute_repay(ctx: Context<RepayContext>) -> Result<()> {
        // Loan logic
        let mut loan = LoanAccount::try_from(&ctx.accounts.account_a.to_account_info())?;
        loan.amount = loan.amount.checked_add(100).unwrap();
        Ok(())
    }
}

#[derive(Accounts)]
pub struct RepayContext<'info> {
    /// CHECK: expecting RepayAccount but using UncheckedAccount
    pub account_a: UncheckedAccount<'info>,
    /// CHECK: expecting RepayAccount but using UncheckedAccount
    pub account_b: UncheckedAccount<'info>,
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct RepayAccount {
    pub dummy: u64,
    pub counter: u64,
    pub version: u8,
}