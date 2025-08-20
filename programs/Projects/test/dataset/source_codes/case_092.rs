use anchor_lang::prelude::*;
declare_id!("Case0921111111111111111111111111111111111111");

#[program]
pub mod case092 {
    use super::*;
    pub fn execute_share_medical_data(ctx: Context<ShareMedicalDataContext>) -> Result<()> {
        // Use Case 92: 医療データ共有（ShareMedicalData）
        // Vulnerable: using UncheckedAccount where ShareMedicalDataAccount is expected
        msg!("Executing execute_share_medical_data for 医療データ共有（ShareMedicalData）");
        // Example logic (dummy operation)
        let mut acct_data = ShareMedicalDataAccount::try_from(&ctx.accounts.account_a.to_account_info())?;
        acct_data.dummy = acct_data.dummy.checked_add(1).unwrap();
        Ok(())
    }
}

#[derive(Accounts)]
pub struct ShareMedicalDataContext<'info> {
    /// CHECK: expecting ShareMedicalDataAccount but using UncheckedAccount
    pub account_a: UncheckedAccount<'info>,
    /// CHECK: expecting ShareMedicalDataAccount but using UncheckedAccount
    pub account_b: UncheckedAccount<'info>,
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct ShareMedicalDataAccount {
    pub dummy: u64,
}