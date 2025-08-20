
// =====================================
// 6. Staking Program
// =====================================
use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};

declare_id!("66666666666666666666666666666666");

#[program]
pub mod secure_staking {
    use super::*;

    pub fn stake_tokens(ctx: Context<StakeTokens>, amount: u64) -> Result<()> {
        // 厳密なowner checkを実装
        require!(
            ctx.accounts.user_token_account.owner == &token::ID,
            ErrorCode::InvalidUserTokenOwner
        );
        require!(
            ctx.accounts.stake_pool_account.owner == &token::ID,
            ErrorCode::InvalidStakePoolOwner
        );
        
        let stake_info = ctx.accounts.stake_info.to_account_info();
        require!(
            stake_info.owner == ctx.program_id,
            ErrorCode::InvalidStakeInfoOwner
        );

        let stake_info = &mut ctx.accounts.stake_info;
        stake_info.staker = ctx.accounts.staker.key();
        stake_info.amount += amount;
        stake_info.last_stake_time = Clock::get()?.unix_timestamp;

        let transfer_instruction = Transfer {
            from: ctx.accounts.user_token_account.to_account_info(),
            to: ctx.accounts.stake_pool_account.to_account_info(),
            authority: ctx.accounts.staker.to_account_info(),
        };

        let cpi_ctx = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            transfer_instruction,
        );

        token::transfer(cpi_ctx, amount)
    }

    pub fn unstake_tokens(ctx: Context<UnstakeTokens>, amount: u64) -> Result<()> {
        // AccountInfoを使った安全なowner check
        let stake_info_account = ctx.accounts.stake_info.to_account_info();
        require!(
            stake_info_account.owner == ctx.program_id,
            ErrorCode::InvalidStakeInfoOwner
        );

        let stake_info = &mut ctx.accounts.stake_info;
        require!(
            stake_info.amount >= amount,
            ErrorCode::InsufficientStakedAmount
        );

        stake_info.amount -= amount;

        let seeds = &[
            b"stake_pool",
            &[ctx.accounts.stake_pool.bump],
        ];
        let signer = &[&seeds[..]];

        let transfer_instruction = Transfer {
            from: ctx.accounts.stake_pool_account.to_account_info(),
            to: ctx.accounts.user_token_account.to_account_info(),
            authority: ctx.accounts.stake_pool.to_account_info(),
        };

        let cpi_ctx = CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            transfer_instruction,
            signer,
        );

        token::transfer(cpi_ctx, amount)
    }
}

#[derive(Accounts)]
pub struct StakeTokens<'info> {
    #[account(
        init_if_needed,
        payer = staker,
        space = 8 + 32 + 8 + 8,
        seeds = [b"stake_info", staker.key().as_ref()],
        bump,
        constraint = stake_info.to_account_info().owner == program_id
    )]
    pub stake_info: Account<'info, StakeInfo>,
    #[account(constraint = stake_pool.to_account_info().owner == program_id)]
    pub stake_pool: Account<'info, StakePool>,
    #[account(
        mut,
        constraint = user_token_account.owner == &token::ID
    )]
    pub user_token_account: Account<'info, TokenAccount>,
    #[account(
        mut,
        constraint = stake_pool_account.owner == &token::ID
    )]
    pub stake_pool_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub staker: Signer<'info>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UnstakeTokens<'info> {
    #[account(
        mut,
        has_one = staker,
        constraint = stake_info.to_account_info().owner == program_id
    )]
    pub stake_info: Account<'info, StakeInfo>,
    #[account(constraint = stake_pool.to_account_info().owner == program_id)]
    pub stake_pool: Account<'info, StakePool>,
    #[account(
        mut,
        constraint = user_token_account.owner == &token::ID
    )]
    pub user_token_account: Account<'info, TokenAccount>,
    #[account(
        mut,
        constraint = stake_pool_account.owner == &token::ID
    )]
    pub stake_pool_account: Account<'info, TokenAccount>,
    pub staker: Signer<'info>,
    pub token_program: Program<'info, Token>,
}

#[account]
pub struct StakeInfo {
    pub staker: Pubkey,
    pub amount: u64,
    pub last_stake_time: i64,
}

#[account]
pub struct StakePool {
    pub admin: Pubkey,
    pub bump: u8,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Invalid user token account owner")]
    InvalidUserTokenOwner,
    #[msg("Invalid stake pool account owner")]
    InvalidStakePoolOwner,
    #[msg("Invalid stake info account owner")]
    InvalidStakeInfoOwner,
    #[msg("Insufficient staked amount")]
    InsufficientStakedAmount,
}
