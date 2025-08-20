use anchor_lang::prelude::*;
declare_id!("FLAS0871111111111111111111111111111111111111");

#[program]
pub mod case087 {
    use super::*;
    pub fn execute_flashloanborrow(ctx: Context<FlashLoanBorrowContext>) -> Result<()> {
        // Loan logic
        let mut loan = LoanAccount::try_from(&ctx.accounts.account_a.to_account_info())?;
        loan.amount = loan.amount.checked_add(100).unwrap();
        Ok(())
    }
}

#[derive(Accounts)]
pub struct FlashLoanBorrowContext<'info> {
    /// CHECK: expecting FlashLoanBorrowAccount but using UncheckedAccount
    pub account_a: UncheckedAccount<'info>,
    /// CHECK: expecting FlashLoanBorrowAccount but using UncheckedAccount
    pub account_b: UncheckedAccount<'info>,
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct FlashLoanBorrowAccount {
    pub dummy: u64,
    pub counter: u64,
    pub version: u8,
}