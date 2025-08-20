use anchor_lang::prelude::*;
declare_id!("Case0911111111111111111111111111111111111111");

#[program]
pub mod case091 {
    use super::*;
    pub fn execute_disburse_scholarship(ctx: Context<DisburseScholarshipContext>) -> Result<()> {
        // Use Case 91: 教育トークン奨学金支給（DisburseScholarship）
        // Vulnerable: using UncheckedAccount where DisburseScholarshipAccount is expected
        msg!("Executing execute_disburse_scholarship for 教育トークン奨学金支給（DisburseScholarship）");
        // Example logic (dummy operation)
        let mut acct_data = DisburseScholarshipAccount::try_from(&ctx.accounts.account_a.to_account_info())?;
        acct_data.dummy = acct_data.dummy.checked_add(1).unwrap();
        Ok(())
    }
}

#[derive(Accounts)]
pub struct DisburseScholarshipContext<'info> {
    /// CHECK: expecting DisburseScholarshipAccount but using UncheckedAccount
    pub account_a: UncheckedAccount<'info>,
    /// CHECK: expecting DisburseScholarshipAccount but using UncheckedAccount
    pub account_b: UncheckedAccount<'info>,
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DisburseScholarshipAccount {
    pub dummy: u64,
}