use anchor_lang::prelude::*;
declare_id!("WRIT0671111111111111111111111111111111111111");

#[program]
pub mod case067 {
    use super::*;
    pub fn execute_writemedicalrecord(ctx: Context<WriteMedicalRecordContext>) -> Result<()> {
        // Medical data access
        msg!("Accessing medical record");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct WriteMedicalRecordContext<'info> {
    /// CHECK: expecting WriteMedicalRecordAccount but using UncheckedAccount
    pub account_a: UncheckedAccount<'info>,
    /// CHECK: expecting WriteMedicalRecordAccount but using UncheckedAccount
    pub account_b: UncheckedAccount<'info>,
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct WriteMedicalRecordAccount {
    pub dummy: u64,
    pub counter: u64,
    pub version: u8,
}