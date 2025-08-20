use anchor_lang::prelude::*;
declare_id!("LOCK0131111111111111111111111111111111111111");

#[program]
pub mod case013 {
    use super::*;
    pub fn execute_locklp(ctx: Context<LockLPContext>) -> Result<()> {
        // Lock LP tokens
        let mut lp = LPAccount::try_from(&ctx.accounts.account_b.to_account_info())?;
        lp.locked = lp.locked.checked_add(250).unwrap();
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
    pub counter: u64,
    pub version: u8,
}