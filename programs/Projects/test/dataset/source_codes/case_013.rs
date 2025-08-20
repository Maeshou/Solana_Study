use anchor_lang::prelude::*;
declare_id!("Case0131111111111111111111111111111111111111");

#[program]
pub mod case013 {
    use super::*;
    pub fn execute_lock_lp(ctx: Context<LockLPContext>) -> Result<()> {
        // Use Case 13: ファーミング LP トークンロック（LockLP）
        // Vulnerable: using UncheckedAccount where LockLPAccount is expected
        msg!("Executing execute_lock_lp for ファーミング LP トークンロック（LockLP）");
        // Example logic (dummy operation)
        let mut acct_data = LockLPAccount::try_from(&ctx.accounts.account_a.to_account_info())?;
        acct_data.dummy = acct_data.dummy.checked_add(1).unwrap();
        Ok(())
    }
}

#[derive(Accounts)]
pub struct LockLPContext<'info> {
    /// CHECK: expecting LockLPAccount but using UncheckedAccount
    pub account_a: UncheckedAccount<'info>,
    /// CHECK: expecting LockLPAccount but using UncheckedAccount
    pub account_b: UncheckedAccount<'info>,
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct LockLPAccount {
    pub dummy: u64,
}