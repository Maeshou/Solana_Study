use anchor_lang::prelude::*;
declare_id!("Case0901111111111111111111111111111111111111");

#[program]
pub mod case090 {
    use super::*;
    pub fn execute_scholarship(ctx: Context<ScholarshipContext>) -> Result<()> {
        // Use Case 90: 教育トークン奨学金（Scholarship）申請
        // Vulnerable: using UncheckedAccount where ScholarshipAccount is expected
        msg!("Executing execute_scholarship for 教育トークン奨学金（Scholarship）申請");
        // Example logic (dummy operation)
        let mut acct_data = ScholarshipAccount::try_from(&ctx.accounts.account_a.to_account_info())?;
        acct_data.dummy = acct_data.dummy.checked_add(1).unwrap();
        Ok(())
    }
}

#[derive(Accounts)]
pub struct ScholarshipContext<'info> {
    /// CHECK: expecting ScholarshipAccount but using UncheckedAccount
    pub account_a: UncheckedAccount<'info>,
    /// CHECK: expecting ScholarshipAccount but using UncheckedAccount
    pub account_b: UncheckedAccount<'info>,
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct ScholarshipAccount {
    pub dummy: u64,
}