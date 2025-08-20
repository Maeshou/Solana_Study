use anchor_lang::prelude::*;
declare_id!("Case0931111111111111111111111111111111111111");

#[program]
pub mod case093 {
    use super::*;
    pub fn execute_fetch_medical_data(ctx: Context<FetchMedicalDataContext>) -> Result<()> {
        // Use Case 93: 医療データ取得（FetchMedicalData）
        // Vulnerable: using UncheckedAccount where FetchMedicalDataAccount is expected
        msg!("Executing execute_fetch_medical_data for 医療データ取得（FetchMedicalData）");
        // Example logic (dummy operation)
        let mut acct_data = FetchMedicalDataAccount::try_from(&ctx.accounts.account_a.to_account_info())?;
        acct_data.dummy = acct_data.dummy.checked_add(1).unwrap();
        Ok(())
    }
}

#[derive(Accounts)]
pub struct FetchMedicalDataContext<'info> {
    /// CHECK: expecting FetchMedicalDataAccount but using UncheckedAccount
    pub account_a: UncheckedAccount<'info>,
    /// CHECK: expecting FetchMedicalDataAccount but using UncheckedAccount
    pub account_b: UncheckedAccount<'info>,
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct FetchMedicalDataAccount {
    pub dummy: u64,
}