use anchor_lang::prelude::*;
declare_id!("Case0601111111111111111111111111111111111111");

#[program]
pub mod case060 {
    use super::*;
    pub fn execute_case60(ctx: Context<Case60Context>) -> Result<()> {
        // Use Case 60: ポイントシステムポイント消費
        // Vulnerable: using UncheckedAccount where Case60Account is expected
        msg!("Executing execute_case60 for ポイントシステムポイント消費");
        // Example logic (dummy operation)
        let mut acct_data = Case60Account::try_from(&ctx.accounts.account_a.to_account_info())?;
        acct_data.dummy = acct_data.dummy.checked_add(1).unwrap();
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Case60Context<'info> {
    /// CHECK: expecting Case60Account but using UncheckedAccount
    pub account_a: UncheckedAccount<'info>,
    /// CHECK: expecting Case60Account but using UncheckedAccount
    pub account_b: UncheckedAccount<'info>,
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Case60Account {
    pub dummy: u64,
}