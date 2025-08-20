use anchor_lang::prelude::*;
declare_id!("SHAR0921111111111111111111111111111111111111");

#[program]
pub mod case092 {
    use super::*;
    pub fn execute_sharemedicaldata(ctx: Context<ShareMedicalDataContext>) -> Result<()> {
        // Default context logic
        msg!("Case 92 executed");
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
    pub counter: u64,
    pub version: u8,
}