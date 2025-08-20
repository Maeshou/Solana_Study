use anchor_lang::prelude::*;
declare_id!("DISB0911111111111111111111111111111111111111");

#[program]
pub mod case091 {
    use super::*;
    pub fn execute_disbursescholarship(ctx: Context<DisburseScholarshipContext>) -> Result<()> {
        // Education credential logic
        let mut cred = CredentialAccount::try_from(&ctx.accounts.account_a.to_account_info())?;
        cred.valid = true;
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
    pub counter: u64,
    pub version: u8,
}