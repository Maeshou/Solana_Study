use anchor_lang::prelude::*;
declare_id!("CLAI0581111111111111111111111111111111111111");

#[program]
pub mod case058 {
    use super::*;
    pub fn execute_claimreferralreward(ctx: Context<ClaimReferralRewardContext>) -> Result<()> {
        // Claim reward based on timestamp
        let now = anchor_lang::solana_program::clock::Clock::get()?.unix_timestamp as u64;
        let reward = now.checked_rem(500).unwrap_or(0);
        **ctx.accounts.account_b.to_account_info().lamports.borrow_mut() += reward;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct ClaimReferralRewardContext<'info> {
    /// CHECK: expecting ClaimReferralRewardAccount but using UncheckedAccount
    pub account_a: UncheckedAccount<'info>,
    /// CHECK: expecting ClaimReferralRewardAccount but using UncheckedAccount
    pub account_b: UncheckedAccount<'info>,
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct ClaimReferralRewardAccount {
    pub dummy: u64,
    pub counter: u64,
    pub version: u8,
}