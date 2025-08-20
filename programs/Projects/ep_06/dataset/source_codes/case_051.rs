use anchor_lang::prelude::*;
declare_id!("CLAI0511111111111111111111111111111111111111");

#[program]
pub mod case051 {
    use super::*;
    pub fn execute_claimreward(ctx: Context<ClaimRewardContext>) -> Result<()> {
        // Default context logic
        msg!("Case 51 executed");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct ClaimRewardContext<'info> {
    /// CHECK: expecting ClaimRewardAccount but using UncheckedAccount
    pub account_a: UncheckedAccount<'info>,
    /// CHECK: expecting ClaimRewardAccount but using UncheckedAccount
    pub account_b: UncheckedAccount<'info>,
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct ClaimRewardAccount {
    pub dummy: u64,
    pub counter: u64,
    pub version: u8,
}