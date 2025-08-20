use anchor_lang::prelude::*;
declare_id!("Case0161111111111111111111111111111111111111");

#[program]
pub mod case016 {
    use super::*;
    pub fn execute_lock_dao_funds(ctx: Context<LockDAOFundsContext>) -> Result<()> {
        // Use Case 16: DAO ファンドロック（LockDAOFunds）
        // Vulnerable: using UncheckedAccount where LockDAOFundsAccount is expected
        msg!("Executing execute_lock_dao_funds for DAO ファンドロック（LockDAOFunds）");
        // Example logic (dummy operation)
        let mut acct_data = LockDAOFundsAccount::try_from(&ctx.accounts.account_a.to_account_info())?;
        acct_data.dummy = acct_data.dummy.checked_add(1).unwrap();
        Ok(())
    }
}

#[derive(Accounts)]
pub struct LockDAOFundsContext<'info> {
    /// CHECK: expecting LockDAOFundsAccount but using UncheckedAccount
    pub account_a: UncheckedAccount<'info>,
    /// CHECK: expecting LockDAOFundsAccount but using UncheckedAccount
    pub account_b: UncheckedAccount<'info>,
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct LockDAOFundsAccount {
    pub dummy: u64,
}