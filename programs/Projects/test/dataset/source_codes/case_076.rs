use anchor_lang::prelude::*;
declare_id!("Case0761111111111111111111111111111111111111");

#[program]
pub mod case076 {
    use super::*;
    pub fn execute_insur_tech(ctx: Context<InsurTechContext>) -> Result<()> {
        // Use Case 76: インシュアテック（InsurTech）保険料計算
        // Vulnerable: using UncheckedAccount where InsurTechAccount is expected
        msg!("Executing execute_insur_tech for インシュアテック（InsurTech）保険料計算");
        // Example logic (dummy operation)
        let mut acct_data = InsurTechAccount::try_from(&ctx.accounts.account_a.to_account_info())?;
        acct_data.dummy = acct_data.dummy.checked_add(1).unwrap();
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InsurTechContext<'info> {
    /// CHECK: expecting InsurTechAccount but using UncheckedAccount
    pub account_a: UncheckedAccount<'info>,
    /// CHECK: expecting InsurTechAccount but using UncheckedAccount
    pub account_b: UncheckedAccount<'info>,
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct InsurTechAccount {
    pub dummy: u64,
}