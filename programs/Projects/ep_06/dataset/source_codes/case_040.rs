use anchor_lang::prelude::*;
declare_id!("APPR0401111111111111111111111111111111111111");

#[program]
pub mod case040 {
    use super::*;
    pub fn execute_approveguarantor(ctx: Context<ApproveGuarantorContext>) -> Result<()> {
        // Default context logic
        msg!("Case 40 executed");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct ApproveGuarantorContext<'info> {
    /// CHECK: expecting ApproveGuarantorAccount but using UncheckedAccount
    pub account_a: UncheckedAccount<'info>,
    /// CHECK: expecting ApproveGuarantorAccount but using UncheckedAccount
    pub account_b: UncheckedAccount<'info>,
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct ApproveGuarantorAccount {
    pub dummy: u64,
    pub counter: u64,
    pub version: u8,
}