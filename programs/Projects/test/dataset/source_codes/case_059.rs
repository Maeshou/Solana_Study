use anchor_lang::prelude::*;
declare_id!("Case0591111111111111111111111111111111111111");

#[program]
pub mod case059 {
    use super::*;
    pub fn execute_point_system(ctx: Context<PointSystemContext>) -> Result<()> {
        // Use Case 59: ポイントシステム（PointSystem）ポイント付与
        // Vulnerable: using UncheckedAccount where PointSystemAccount is expected
        msg!("Executing execute_point_system for ポイントシステム（PointSystem）ポイント付与");
        // Example logic (dummy operation)
        let mut acct_data = PointSystemAccount::try_from(&ctx.accounts.account_a.to_account_info())?;
        acct_data.dummy = acct_data.dummy.checked_add(1).unwrap();
        Ok(())
    }
}

#[derive(Accounts)]
pub struct PointSystemContext<'info> {
    /// CHECK: expecting PointSystemAccount but using UncheckedAccount
    pub account_a: UncheckedAccount<'info>,
    /// CHECK: expecting PointSystemAccount but using UncheckedAccount
    pub account_b: UncheckedAccount<'info>,
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct PointSystemAccount {
    pub dummy: u64,
}