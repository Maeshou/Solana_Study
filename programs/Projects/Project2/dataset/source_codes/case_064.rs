// 4. Staking Program with Reward Distribution
use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount, Mint};

declare_id!("StakingProgram1111111111111111111111111111111111");

#[program]
pub mod staking_program {
    use super::*;
    
    pub fn initialize_pool(ctx: Context<InitializePool>, reward_rate: u64) -> Result<()> {
        let pool = &mut ctx.accounts.pool;
        pool.authority = ctx.accounts.authority.key();
        pool.stake_mint = ctx.accounts.stake_mint.key();
        pool.reward_mint = ctx.accounts.reward_mint.key();
        pool.reward_rate = reward_rate;
        pool.total_staked = 0;
        pool.last_update = Clock::get()?.unix_timestamp;
        Ok(())
    }
    
    pub fn stake_tokens(ctx: Context<StakeTokens>, amount: u64) -> Result<()> {
        let pool = &mut ctx.accounts.pool;
        let user_stake = &mut ctx.accounts.user_stake;
        
        // Update pool rewards before staking
        pool.last_update = Clock::get()?.unix_timestamp;
        
        // Transfer tokens to pool
        let cpi_accounts = anchor_spl::token::Transfer {
            from: ctx.accounts.user_token_account.to_account_info(),
            to: ctx.accounts.pool_token_account.to_account_info(),
            authority: ctx.accounts.user.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        
        anchor_spl::token::transfer(cpi_ctx, amount)?;
        
        user_stake.amount += amount;
        user_stake.last_stake_time = Clock::get()?.unix_timestamp;
        pool.total_staked += amount;
        
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializePool<'info> {
    #[account(init, payer = authority, space = 8 + 200)]
    pub pool: Account<'info, StakePool>,
    pub stake_mint: Account<'info, Mint>,
    pub reward_mint: Account<'info, Mint>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct StakeTokens<'info> {
    #[account(mut)]
    pub pool: Account<'info, StakePool>,
    #[account(init_if_needed, payer = user, space = 8 + 100, seeds = [b"user_stake", user.key().as_ref(), pool.key().as_ref()], bump)]
    pub user_stake: Account<'info, UserStake>,
    #[account(mut, constraint = user_token_account.mint == pool.stake_mint)]
    pub user_token_account: Account<'info, TokenAccount>,
    #[account(mut, constraint = pool_token_account.mint == pool.stake_mint)]
    pub pool_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct StakePool {
    pub authority: Pubkey,
    pub stake_mint: Pubkey,
    pub reward_mint: Pubkey,
    pub reward_rate: u64,
    pub total_staked: u64,
    pub last_update: i64,
}

#[account]
pub struct UserStake {
    pub amount: u64,
    pub last_stake_time: i64,
}