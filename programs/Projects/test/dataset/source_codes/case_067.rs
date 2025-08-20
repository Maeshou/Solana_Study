use anchor_lang::prelude::*;
declare_id!("Case0671111111111111111111111111111111111111");

#[program]
pub mod case067 {
    use super::*;
    pub fn execute_medical_record(ctx: Context<MedicalRecordContext>) -> Result<()> {
        // Use Case 67: 医療記録書き込み（MedicalRecord）
        // Vulnerable: using UncheckedAccount where MedicalRecordAccount is expected
        msg!("Executing execute_medical_record for 医療記録書き込み（MedicalRecord）");
        // Example logic (dummy operation)
        let mut acct_data = MedicalRecordAccount::try_from(&ctx.accounts.account_a.to_account_info())?;
        acct_data.dummy = acct_data.dummy.checked_add(1).unwrap();
        Ok(())
    }
}

#[derive(Accounts)]
pub struct MedicalRecordContext<'info> {
    /// CHECK: expecting MedicalRecordAccount but using UncheckedAccount
    pub account_a: UncheckedAccount<'info>,
    /// CHECK: expecting MedicalRecordAccount but using UncheckedAccount
    pub account_b: UncheckedAccount<'info>,
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct MedicalRecordAccount {
    pub dummy: u64,
}