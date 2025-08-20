use anchor_lang::prelude::*;
declare_id!("Case0361111111111111111111111111111111111111");

#[program]
pub mod case036 {
    use super::*;
    pub fn execute_clearing(ctx: Context<ClearingContext>) -> Result<()> {
        // Use Case 36: デリバティブ清算（Clearing）
        // Vulnerable: using UncheckedAccount where ClearingAccount is expected
        msg!("Executing execute_clearing for デリバティブ清算（Clearing）");
        // Example logic (dummy operation)
        let mut acct_data = ClearingAccount::try_from(&ctx.accounts.account_a.to_account_info())?;
        acct_data.dummy = acct_data.dummy.checked_add(1).unwrap();
        Ok(())
    }
}

#[derive(Accounts)]
pub struct ClearingContext<'info> {
    /// CHECK: expecting ClearingAccount but using UncheckedAccount
    pub account_a: UncheckedAccount<'info>,
    /// CHECK: expecting ClearingAccount but using UncheckedAccount
    pub account_b: UncheckedAccount<'info>,
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct ClearingAccount {
    pub dummy: u64,
}