use anchor_lang::prelude::*;
declare_id!("Case0271111111111111111111111111111111111111");

#[program]
pub mod case027 {
    use super::*;
    pub fn execute_borrow(ctx: Context<BorrowContext>) -> Result<()> {
        // Use Case 27: レンディング借入（Borrow）
        // Vulnerable: using UncheckedAccount where BorrowAccount is expected
        msg!("Executing execute_borrow for レンディング借入（Borrow）");
        // Example logic (dummy operation)
        let mut acct_data = BorrowAccount::try_from(&ctx.accounts.account_a.to_account_info())?;
        acct_data.dummy = acct_data.dummy.checked_add(1).unwrap();
        Ok(())
    }
}

#[derive(Accounts)]
pub struct BorrowContext<'info> {
    /// CHECK: expecting BorrowAccount but using UncheckedAccount
    pub account_a: UncheckedAccount<'info>,
    /// CHECK: expecting BorrowAccount but using UncheckedAccount
    pub account_b: UncheckedAccount<'info>,
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct BorrowAccount {
    pub dummy: u64,
}