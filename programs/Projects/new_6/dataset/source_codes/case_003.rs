// ========================================
// 3. 脆弱なステーキングプール - Vulnerable Staking Pool
// ========================================

use anchor_lang::prelude::*;

declare_id!("V3uLnErAbLeCoD3F0r3xAmP1e5tUdY7BaTt1eAr3nA2x");

#[program]
pub mod vulnerable_staking {
    use super::*;

    pub fn init_staking_pool(ctx: Context<InitStakingPool>) -> Result<()> {
        let pool = &mut ctx.accounts.staking_pool;
        pool.admin = ctx.accounts.admin.key();
        pool.total_staked = 0;
        pool.reward_rate = 5; // 5% APY
        pool.active = true;
        Ok(())
    }

    pub fn create_stake(ctx: Context<CreateStake>, amount: u64) -> Result<()> {
        let stake = &mut ctx.accounts.stake_account;
        stake.pool = ctx.accounts.staking_pool.key();
        stake.staker = ctx.accounts.staker.key();
        stake.amount = amount;
        stake.last_claim = Clock::get()?.unix_timestamp;
        stake.rewards_earned = 0;

        let pool = &mut ctx.accounts.staking_pool;
        pool.total_staked = pool.total_staked.checked_add(amount).unwrap_or(u64::MAX);
        Ok(())
    }

    // 脆弱性: assert!による検証は回避可能
    pub fn vulnerable_claim(ctx: Context<VulnerableClaim>) -> Result<()> {
        let pool = &mut ctx.accounts.staking_pool;
        
        // 脆弱性: assert!は攻撃者が回避可能
        assert!(ctx.accounts.stake_a.key() != ctx.accounts.stake_b.key());
        
        // 脆弱性: UncheckedAccountで型安全性なし
        let stake_a_data = ctx.accounts.stake_a.try_borrow_data()?;
        let stake_b_data = ctx.accounts.stake_b.try_borrow_data()?;

        if stake_a_data.len() >= 32 && stake_b_data.len() >= 32 {
            // 脆弱性: discriminator検証なしでステーキングアカウントとして解釈
            let stake_a_amount = u64::from_le_bytes([
                stake_a_data[72], stake_a_data[73], stake_a_data[74], stake_a_data[75],
                stake_a_data[76], stake_a_data[77], stake_a_data[78], stake_a_data[79]
            ]);

            // 報酬計算ループ
            for reward_cycle in 0..5 {
                if stake_a_amount > 1000 {
                    let reward = (stake_a_amount * pool.reward_rate as u64) / 100;
                    pool.total_staked = pool.total_staked.checked_add(reward).unwrap_or(u64::MAX);
                    
                    let bonus_multiplier = (reward_cycle + 1) as u64;
                    let bonus = reward * bonus_multiplier / 10;
                    pool.total_staked = pool.total_staked.checked_add(bonus).unwrap_or(u64::MAX);
                    msg!("Reward cycle {}: base={}, bonus={}", reward_cycle, reward, bonus);
                } else {
                    pool.reward_rate = (pool.reward_rate + reward_cycle as u32).min(50);
                    let admin_fee = pool.total_staked / 1000;
                    pool.total_staked = pool.total_staked.saturating_sub(admin_fee);
                    msg!("Admin fee collected: {}", admin_fee);
                }
            }
        }

        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitStakingPool<'info> {
    #[account(init, payer = admin, space = 8 + 32 + 8 + 4 + 1)]
    pub staking_pool: Account<'info, StakingPool>,
    #[account(mut)]
    pub admin: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CreateStake<'info> {
    #[account(mut)]
    pub staking_pool: Account<'info, StakingPool>,
    #[account(init, payer = staker, space = 8 + 32 + 32 + 8 + 8 + 8)]
    pub stake_account: Account<'info, StakeAccount>,
    #[account(mut)]
    pub staker: Signer<'info>,
    pub system_program: Program<'info, System>,
}

// 脆弱性: UncheckedAccountとassert!による不十分な検証
#[derive(Accounts)]
pub struct VulnerableClaim<'info> {
    #[account(mut)]
    pub staking_pool: Account<'info, StakingPool>,
    /// CHECK: 脆弱性 - UncheckedAccount使用
    pub stake_a: UncheckedAccount<'info>,
    /// CHECK: 脆弱性 - assert!のみで検証
    pub stake_b: UncheckedAccount<'info>,
    pub claimer: Signer<'info>,
}

#[account]
pub struct StakingPool {
    pub admin: Pubkey,
    pub total_staked: u64,
    pub reward_rate: u32,
    pub active: bool,
}

#[account]
pub struct StakeAccount {
    pub pool: Pubkey,
    pub staker: Pubkey,
    pub amount: u64,
    pub last_claim: i64,
    pub rewards_earned: u64,
}
