use anchor_lang::prelude::*;
declare_id!("APPL0901111111111111111111111111111111111111");

#[program]
pub mod case090 {
    use super::*;
    pub fn execute_applyscholarship(ctx: Context<ApplyScholarshipContext>) -> Result<()> {
        // Education credential logic
        let mut cred = CredentialAccount::try_from(&ctx.accounts.account_a.to_account_info())?;
        cred.valid = true;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct ApplyScholarshipContext<'info> {
    /// CHECK: expecting ApplyScholarshipAccount but using UncheckedAccount
    pub account_a: UncheckedAccount<'info>,
    /// CHECK: expecting ApplyScholarshipAccount but using UncheckedAccount
    pub account_b: UncheckedAccount<'info>,
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct ApplyScholarshipAccount {
    pub dummy: u64,
    pub counter: u64,
    pub version: u8,
}