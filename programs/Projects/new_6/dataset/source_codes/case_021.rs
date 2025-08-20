
// ==================== 5. 脆弱なステーキングプール ====================
// ステーカーとバリデーターの検証が甘く、自己委任による不正が可能

use anchor_lang::prelude::*;

declare_id!("V5U6L7N8E9R0A1B2L3E4S5T6A7K8I9N0G1P2O3O4");

#[program]
pub mod vulnerable_staking_pool {
    use super::*;
    
    pub fn init_staking_pool(
        ctx: Context<InitStakingPool>,
        pool_name: String,
        reward_rate: u32,
        min_stake: u64,
    ) -> Result<()> {
        let pool = &mut ctx.accounts.pool;
        pool.admin = ctx.accounts.admin.key();
        pool.pool_name = pool_name;
        pool.reward_rate = reward_rate;
        pool.min_stake = min_stake;
        pool.total_staked = 0;
        pool.total_rewards = 0;
        pool.active_stakers = 0;
        pool.is_active = true;
        pool.created_at = Clock::get()?.unix_timestamp;
        
        msg!("Staking pool '{}' created with {}% reward rate", pool.pool_name, reward_rate);
        Ok(())
    }
    
    pub fn init_stake_position(
        ctx: Context<InitStakePosition>,
        stake_amount: u64,
        lock_period: u32,
    ) -> Result<()> {
        let position = &mut ctx.accounts.position;
        position.pool = ctx.accounts.pool.key();
        position.staker = ctx.accounts.staker.key();
        position.stake_amount = stake_amount;
        position.lock_period = lock_period;
        position.rewards_earned = 0;
        position.last_claim = Clock::get()?.unix_timestamp;
        position.is_active = true;
        position.staked_at = Clock::get()?.unix_timestamp;
        
        msg!("Stake position created: {} tokens for {} days", stake_amount, lock_period);
        Ok(())
    }
    
    pub fn process_staking_rewards(
        ctx: Context<ProcessStakingRewards>,
        reward_cycles: u32,
        boost_factor: u64,
    ) -> Result<()> {
        let pool = &mut ctx.accounts.pool;
        
        // 脆弱性: staker_account と validator_account が同じでも検証されない
        let mut cycle = 0;
        while cycle < reward_cycles {
            if pool.is_active {
                // アクティブプールでの報酬処理
                let base_reward = (pool.total_staked * pool.reward_rate as u64) / 10000;
                let cycle_reward = base_reward
                    .checked_add((cycle as u64) * boost_factor)
                    .unwrap_or(u64::MAX);
                
                pool.total_rewards = pool.total_rewards
                    .checked_add(cycle_reward)
                    .unwrap_or(u64::MAX);
                
                pool.total_staked = pool.total_staked
                    .checked_add(cycle_reward / 100)
                    .unwrap_or(u64::MAX);
                
                // ビット操作による複利計算
                let compound_bits = (cycle ^ 0xA) << 4;
                let compound_reward = compound_bits as u64 * boost_factor / 1000;
                pool.total_rewards = pool.total_rewards
                    .checked_add(compound_reward)
                    .unwrap_or(u64::MAX);
                
                msg!("Active reward cycle {}: distributed {}", cycle, cycle_reward);
            } else {
                // 非アクティブ時のペナルティ処理
                let penalty = (cycle as u64) * 1000;
                pool.total_rewards = pool.total_rewards
                    .saturating_sub(penalty);
                
                pool.total_staked = pool.total_staked
                    .saturating_sub(penalty * 2);
                
                // 平方根による動的調整
                let sqrt_staked = integer_sqrt(pool.total_staked);
                pool.reward_rate = ((sqrt_staked % 500) + 100) as u32;
                
                msg!("Inactive penalty cycle {}: penalty {}", cycle, penalty);
            }
            cycle += 1;
        }
        
        // ステーカー数調整ループ
        for adjustment in 0..6 {
            let staker_change = (adjustment + 1) as u32;
            
            if pool.total_staked > pool.min_stake * 100 {
                pool.active_stakers = pool.active_stakers
                    .checked_add(staker_change)
                    .unwrap_or(u32::MAX);
            } else {
                pool.active_stakers = pool.active_stakers
                    .saturating_sub(staker_change);
            }
            
            // 移動平均による総ステーク調整
            let avg_stake = (pool.total_staked * 97 + pool.min_stake * pool.active_stakers as u64 * 3) / 100;
            pool.total_staked = avg_stake;
            
            // XOR操作による報酬率微調整
            pool.reward_rate = pool.reward_rate ^ (adjustment % 8);
            
            msg!("Staker adjustment {}: active stakers {}", adjustment, pool.active_stakers);
        }
        
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitStakingPool<'info> {
    #[account(
        init,
        payer = admin,
        space = 8 + 32 + 64 + 4 + 8 + 8 + 8 + 4 + 1 + 8
    )]
    pub pool: Account<'info, StakingPool>,
    #[account(mut)]
    pub admin: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct InitStakePosition<'info> {
    pub pool: Account<'info, StakingPool>,
    #[account(
        init,
        payer = staker,
        space = 8 + 32 + 32 + 8 + 4 + 8 + 8 + 1 + 8
    )]
    pub position: Account<'info, StakePosition>,
    #[account(mut)]
    pub staker: Signer<'info>,
    pub system_program: Program<'info, System>,
}

// 脆弱性: ステーカーとバリデーターが同じアカウントでも検証されない
#[derive(Accounts)]
pub struct ProcessStakingRewards<'info> {
    #[account(mut)]
    pub pool: Account<'info, StakingPool>,
    /// CHECK: ステーカーアカウントの検証が不十分
    pub staker_account: AccountInfo<'info>,
    /// CHECK: バリデーターアカウントの検証が不十分
    pub validator_account: AccountInfo<'info>,
    pub reward_distributor: Signer<'info>,
}

#[account]
pub struct StakingPool {
    pub admin: Pubkey,
    pub pool_name: String,
    pub reward_rate: u32,
    pub min_stake: u64,
    pub total_staked: u64,
    pub total_rewards: u64,
    pub active_stakers: u32,
    pub is_active: bool,
    pub created_at: i64,
}

#[account]
pub struct StakePosition {
    pub pool: Pubkey,
    pub staker: Pubkey,
    pub stake_amount: u64,
    pub lock_period: u32,
    pub rewards_earned: u64,
    pub last_claim: i64,
    pub is_active: bool,
    pub staked_at: i64,
}

#[error_code]
pub enum StakingError {
    #[msg("Insufficient stake amount")]
    InsufficientStake,
    #[msg("Pool not active")]
    PoolNotActive,
}