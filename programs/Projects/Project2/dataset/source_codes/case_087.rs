use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount, Transfer};

declare_id!("StakePool1818181818181818181818181818181818");

#[program]
pub mod stake_pool_manager {
    use super::*;

    /// プール生成：パラメータ設定
    pub fn init_pool(ctx: Context<InitPool>, fee_bps: u16) -> Result<()> {
        let pool = &mut ctx.accounts.pool;
        pool.fee_basis = fee_bps;
        pool.total_staked = 0;
        emit!(PoolCreated { fee_bps });
        Ok(())
    }

    /// ステーク：ユーザーからトークン受領し記録
    pub fn stake(ctx: Context<Stake>, amount: u64) -> Result<()> {
        let cpi = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.user_token.to_account_info(),
                to: ctx.accounts.pool_token.to_account_info(),
                authority: ctx.accounts.user.to_account_info(),
            },
        );
        anchor_spl::token::transfer(cpi, amount)?;
        let pool = &mut ctx.accounts.pool;
        pool.total_staked = pool.total_staked.checked_add(amount).unwrap();
        emit!(Staked { user: ctx.accounts.user.key(), amount });
        Ok(())
    }

    /// 解除：手数料差引き後に返却
    pub fn unstake(ctx: Context<Stake>, amount: u64) -> Result<()> {
        let pool = &mut ctx.accounts.pool;
        require!(pool.total_staked >= amount, ErrorCode::Insufficient);
        let fee = amount * pool.fee_basis as u64 / 10_000;
        let net = amount.checked_sub(fee).unwrap();
        **ctx.accounts.user_token.to_account_info().lamports.borrow_mut() += net;
        pool.total_staked = pool.total_staked.checked_sub(amount).unwrap();
        emit!(Unstaked { user: ctx.accounts.user.key(), amount: net });
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitPool<'info> {
    #[account(init, payer = payer, space = 8 + 2 + 8)]
    pub pool: Account<'info, StakePool>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Stake<'info> {
    #[account(mut)]
    pub pool: Account<'info, StakePool>,
    #[account(mut)]
    pub user_token: Account<'info, TokenAccount>,
    #[account(mut)]
    pub pool_token: Account<'info, TokenAccount>,
    pub user: Signer<'info>,
    pub token_program: Program<'info, Token>,
}

#[account]
pub struct StakePool {
    pub fee_basis: u16,
    pub total_staked: u64,
}

#[event]
pub struct PoolCreated {
    pub fee_bps: u16,
}

#[event]
pub struct Staked {
    pub user: Pubkey,
    pub amount: u64,
}

#[event]
pub struct Unstaked {
    pub user: Pubkey,
    pub amount: u64,
}

#[error_code]
pub enum ErrorCode {
    #[msg("残高不足")]
    Insufficient,
}
