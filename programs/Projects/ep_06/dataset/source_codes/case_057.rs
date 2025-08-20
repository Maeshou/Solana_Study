use anchor_lang::prelude::*;
declare_id!("REGI0571111111111111111111111111111111111111");

#[program]
pub mod case057 {
    use super::*;
    pub fn execute_registerreferral(ctx: Context<RegisterReferralContext>) -> Result<()> {
        // Default context logic
        msg!("Case 57 executed");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct RegisterReferralContext<'info> {
    /// CHECK: expecting RegisterReferralAccount but using UncheckedAccount
    pub account_a: UncheckedAccount<'info>,
    /// CHECK: expecting RegisterReferralAccount but using UncheckedAccount
    pub account_b: UncheckedAccount<'info>,
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct RegisterReferralAccount {
    pub dummy: u64,
    pub counter: u64,
    pub version: u8,
}