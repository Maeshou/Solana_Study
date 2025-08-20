use anchor_lang::prelude::*;
use std::collections::BTreeMap;

declare_id!("StakeRwd333333333333333333333333333333333");

#[program]
pub mod staking_rewards {
    use super::*;

    /// ステーク量更新
    pub fn stake(ctx: Context<Stake>, amount: u64) -> Result<()> {
        let s = &mut ctx.accounts.staking;
        *s.stakes.entry(ctx.accounts.user.key()).or_insert(0) += amount;
        let entry = s.rewards.entry(ctx.accounts.user.key()).or_insert(0);
        *entry = entry.saturating_add(amount / s.rate_divisor);
        s.total_staked = s.total_staked.saturating_add(amount);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Stake<'info> {
    #[account(mut)]
    pub staking: Account<'info, StakingData>,
    pub user: Signer<'info>,
}

#[account]
pub struct StakingData {
    pub stakes: BTreeMap<Pubkey, u64>,
    pub rewards: BTreeMap<Pubkey, u64>,
    pub total_staked: u64,
    pub rate_divisor: u64,
}
