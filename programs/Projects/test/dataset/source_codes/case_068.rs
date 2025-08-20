use anchor_lang::prelude::*;
declare_id!("Case0681111111111111111111111111111111111111");

#[program]
pub mod case068 {
    use super::*;
    pub fn execute_edu_credential(ctx: Context<EduCredentialContext>) -> Result<()> {
        // Use Case 68: 教育認証発行（EduCredential）
        // Vulnerable: using UncheckedAccount where EduCredentialAccount is expected
        msg!("Executing execute_edu_credential for 教育認証発行（EduCredential）");
        // Example logic (dummy operation)
        let mut acct_data = EduCredentialAccount::try_from(&ctx.accounts.account_a.to_account_info())?;
        acct_data.dummy = acct_data.dummy.checked_add(1).unwrap();
        Ok(())
    }
}

#[derive(Accounts)]
pub struct EduCredentialContext<'info> {
    /// CHECK: expecting EduCredentialAccount but using UncheckedAccount
    pub account_a: UncheckedAccount<'info>,
    /// CHECK: expecting EduCredentialAccount but using UncheckedAccount
    pub account_b: UncheckedAccount<'info>,
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct EduCredentialAccount {
    pub dummy: u64,
}