use anchor_lang::prelude::*;
declare_id!("SETG0391111111111111111111111111111111111111");

#[program]
pub mod case039 {
    use super::*;
    pub fn execute_setguarantor(ctx: Context<SetGuarantorContext>) -> Result<()> {
        // Default context logic
        msg!("Case 39 executed");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct SetGuarantorContext<'info> {
    /// CHECK: expecting SetGuarantorAccount but using UncheckedAccount
    pub account_a: UncheckedAccount<'info>,
    /// CHECK: expecting SetGuarantorAccount but using UncheckedAccount
    pub account_b: UncheckedAccount<'info>,
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct SetGuarantorAccount {
    pub dummy: u64,
    pub counter: u64,
    pub version: u8,
}