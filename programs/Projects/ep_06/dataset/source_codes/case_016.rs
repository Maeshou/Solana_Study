use anchor_lang::prelude::*;
declare_id!("LOCK0161111111111111111111111111111111111111");

#[program]
pub mod case016 {
    use super::*;
    pub fn execute_lockdaofunds(ctx: Context<LockDAOFundsContext>) -> Result<()> {
        // DAO funds lock
        let mut dao = DAOAccount::try_from(&ctx.accounts.account_b.to_account_info())?;
        dao.locked_funds = dao.locked_funds.checked_add(10000).unwrap();
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
    pub counter: u64,
    pub version: u8,
}