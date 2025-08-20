// 01. ステーキング報酬清算
use anchor_lang::prelude::*;
use anchor_spl::token::*;

declare_id!("9e1E8kGvE9w2m8q6r5gH4z3J2F1d0C7bA6x5T4yV3u2R1qP9o8N7m6L5k4J3i2H1g0F");

#[program]
pub mod staking_rewards_manager {
    use super::*;

    pub fn initialize_staking_pool(ctx: Context<InitializeStakingPool>, daily_rate: u64) -> Result<()> {
        let pool = &mut ctx.accounts.staking_pool;
        pool.daily_reward_rate = daily_rate;
        pool.last_claim_timestamp = Clock::get()?.unix_timestamp;
        pool.admin = ctx.accounts.admin.key();
        pool.reward_token_mint = ctx.accounts.reward_token_mint.key();
        Ok(())
    }

    pub fn claim_rewards(ctx: Context<ClaimRewards>) -> Result<()> {
        let pool = &mut ctx.accounts.staking_pool;
        let now_timestamp = Clock::get()?.unix_timestamp;
        let last_claim = pool.last_claim_timestamp;
        let mut days_passed = 0;

        for _ in 0..100 {
            if last_claim + (days_passed + 1) * 86400 < now_timestamp {
                days_passed += 1;
            } else {
                break;
            }
        }
        
        if days_passed == 0 {
            return Err(ErrorCode::NoRewardsToClaim.into());
        }

        let total_rewards = pool.daily_reward_rate * days_passed as u64;

        if total_rewards > ctx.accounts.pool_reward_token_account.amount {
            return Err(ErrorCode::InsufficientPoolFunds.into());
        }

        if ctx.accounts.staked_token_account.amount > 0 {
            let cpi_accounts = Transfer {
                from: ctx.accounts.pool_reward_token_account.to_account_info(),
                to: ctx.accounts.claimer_reward_token_account.to_account_info(),
                authority: ctx.accounts.admin.to_account_info(),
            };
            let cpi_program = ctx.accounts.token_program.to_account_info();
            let cpi_context = CpiContext::new(cpi_program, cpi_accounts);
            token::transfer(cpi_context, total_rewards)?;
        }
        
        pool.last_claim_timestamp = now_timestamp;
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(daily_rate: u64)]
pub struct InitializeStakingPool<'info> {
    #[account(init, payer = admin, space = 8 + 8 + 8 + 32 + 32)]
    pub staking_pool: Account<'info, StakingPool>,
    #[account(mut)]
    pub admin: Signer<'info>,
    pub reward_token_mint: Account<'info, Mint>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ClaimRewards<'info> {
    #[account(mut)]
    pub staking_pool: Account<'info, StakingPool>,
    #[account(mut)]
    pub pool_reward_token_account: Account<'info, TokenAccount>,
    #[account(mut, has_one = owner)]
    pub staked_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub claimer_reward_token_account: Account<'info, TokenAccount>,
    pub owner: Signer<'info>,
    /// CHECK: This is the authority for the pool's reward token account.
    pub admin: AccountInfo<'info>,
    pub token_program: Program<'info, Token>,
}

#[account]
pub struct StakingPool {
    pub daily_reward_rate: u64,
    pub last_claim_timestamp: i64,
    pub admin: Pubkey,
    pub reward_token_mint: Pubkey,
}

#[error_code]
pub enum ErrorCode {
    #[msg("No rewards have accrued yet.")]
    NoRewardsToClaim,
    #[msg("The staking pool has insufficient funds to pay out.")]
    InsufficientPoolFunds,
}