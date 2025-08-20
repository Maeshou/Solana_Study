use anchor_lang::prelude::*;
declare_id!("Case0331111111111111111111111111111111111111");

#[program]
pub mod case033 {
    use super::*;
    pub fn execute_case33(ctx: Context<Case33Context>) -> Result<()> {
        // Use Case 33: 分散型保存読み込み
        // Vulnerable: using UncheckedAccount where Case33Account is expected
        msg!("Executing execute_case33 for 分散型保存読み込み");
        // Example logic (dummy operation)
        let mut acct_data = Case33Account::try_from(&ctx.accounts.account_a.to_account_info())?;
        acct_data.dummy = acct_data.dummy.checked_add(1).unwrap();
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Case33Context<'info> {
    /// CHECK: expecting Case33Account but using UncheckedAccount
    pub account_a: UncheckedAccount<'info>,
    /// CHECK: expecting Case33Account but using UncheckedAccount
    pub account_b: UncheckedAccount<'info>,
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Case33Account {
    pub dummy: u64,
}