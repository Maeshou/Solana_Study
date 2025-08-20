use anchor_lang::prelude::*;
declare_id!("Case0531111111111111111111111111111111111111");

#[program]
pub mod case053 {
    use super::*;
    pub fn execute_一括(ctx: Context<一括Context>) -> Result<()> {
        // Use Case 53: トークンミント（一括）
        // Vulnerable: using UncheckedAccount where 一括Account is expected
        msg!("Executing execute_一括 for トークンミント（一括）");
        // Example logic (dummy operation)
        let mut acct_data = 一括Account::try_from(&ctx.accounts.account_a.to_account_info())?;
        acct_data.dummy = acct_data.dummy.checked_add(1).unwrap();
        Ok(())
    }
}

#[derive(Accounts)]
pub struct 一括Context<'info> {
    /// CHECK: expecting 一括Account but using UncheckedAccount
    pub account_a: UncheckedAccount<'info>,
    /// CHECK: expecting 一括Account but using UncheckedAccount
    pub account_b: UncheckedAccount<'info>,
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct 一括Account {
    pub dummy: u64,
}