// 09. Staking Pool - Validator vs Delegator confusion
use anchor_lang::prelude::*;

declare_id!("StakingPool99999999999999999999999999999999999");

#[program]
pub mod staking_pool {
    use super::*;

    pub fn init_staking_pool(ctx: Context<InitStakingPool>, commission_rate: u16, min_stake: u64) -> Result<()> {
        let pool = &mut ctx.accounts.staking_pool;
        pool.validator = ctx.accounts.validator.key();
        pool.commission_rate = commission_rate;
        pool.minimum_stake = min_stake;
        pool.total_staked = 0;
        pool.total_rewards = 0;
        pool.delegator_count = 0;
        pool.epoch_rewards = [0; 10];
        pool.is_active = true;
        Ok(())
    }

    pub fn distribute_rewards(ctx: Context<DistributeRewards>, epoch_reward: u64) -> Result<()> {
        let pool = &mut ctx.accounts.staking_pool;
        let distributor = &ctx.accounts.distributor;
        
        // Vulnerable: Any account can distribute rewards
        pool.total_rewards += epoch_reward;
        
        // Complex reward distribution with loops
        let validator_commission = (epoch_reward * pool.commission_rate as u64) / 10000;
        let delegator_rewards = epoch_reward - validator_commission;
        
        // Update epoch history
        for i in 1..10 {
            pool.epoch_rewards[i-1] = pool.epoch_rewards[i];
        }
        pool.epoch_rewards[9] = epoch_reward;
        
        // Calculate per-delegator rewards
        if pool.delegator_count > 0 && pool.total_staked > 0 {
            let reward_per_token = (delegator_rewards * 1_000_000) / pool.total_staked;
            
            // Simulate delegator reward updates
            for delegator_idx in 0..pool.delegator_count.min(50) {
                let delegator_stake = pool.total_staked / pool.delegator_count as u64;
                let delegator_reward = (delegator_stake * reward_per_token) / 1_000_000;
                pool.pending_rewards += delegator_reward;
            }
        }
        
        pool.validator_earnings += validator_commission;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitStakingPool<'info> {
    #[account(init, payer = validator, space = 8 + 500)]
    pub staking_pool: Account<'info, StakingPoolData>,
    #[account(mut)]
    pub validator: AccountInfo<'info>, // No validator verification
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct DistributeRewards<'info> {
    #[account(mut)]
    pub staking_pool: Account<'info, StakingPoolData>,
    pub distributor: AccountInfo<'info>, // Could be anyone, not just validator
}

#[account]
pub struct StakingPoolData {
    pub validator: Pubkey,
    pub commission_rate: u16,
    pub minimum_stake: u64,
    pub total_staked: u64,
    pub total_rewards: u64,
    pub delegator_count: u32,
    pub epoch_rewards: [u64; 10],
    pub pending_rewards: u64,
    pub validator_earnings: u64,
    pub is_active: bool,
}
