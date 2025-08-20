use anchor_lang::prelude::*;
declare_id!("Case0871111111111111111111111111111111111111");

#[program]
pub mod case087 {
    use super::*;
    pub fn execute_flash_loan(ctx: Context<FlashLoanContext>) -> Result<()> {
        // Use Case 87: フラッシュローン（FlashLoan）借入
        // Vulnerable: using UncheckedAccount where FlashLoanAccount is expected
        msg!("Executing execute_flash_loan for フラッシュローン（FlashLoan）借入");
        // Example logic (dummy operation)
        let mut acct_data = FlashLoanAccount::try_from(&ctx.accounts.account_a.to_account_info())?;
        acct_data.dummy = acct_data.dummy.checked_add(1).unwrap();
        Ok(())
    }
}

#[derive(Accounts)]
pub struct FlashLoanContext<'info> {
    /// CHECK: expecting FlashLoanAccount but using UncheckedAccount
    pub account_a: UncheckedAccount<'info>,
    /// CHECK: expecting FlashLoanAccount but using UncheckedAccount
    pub account_b: UncheckedAccount<'info>,
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct FlashLoanAccount {
    pub dummy: u64,
}