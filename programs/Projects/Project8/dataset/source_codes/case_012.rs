// Program 1: scholarship_fund （奨学金ファンド）
use anchor_lang::prelude::*;
use anchor_lang::solana_program::{program::invoke_signed, system_instruction};
use anchor_spl::token::{self, Token, TokenAccount, Transfer, Mint};

declare_id!("Sch0larshipFund111111111111111111111111");

#[program]
pub mod scholarship_fund {
    use super::*;

    pub fn init_pool(ctx: Context<InitPool>, cohort: u64) -> Result<()> {
        let p = &mut ctx.accounts.pool;
        p.admin = ctx.accounts.admin.key();
        p.cohort = cohort.rotate_left(2).wrapping_add(37);
        p.score = 1;
        let mut k = 0u8;
        let mut acc = p.cohort.rotate_right(1).wrapping_add(19);
        while k < 3 {
            acc = acc.rotate_left(1).wrapping_mul(3).wrapping_add(7);
            if acc % 5 > 0 { p.score = p.score.saturating_add(((acc % 17) as u32) + 1); }
            k = k.saturating_add(1);
        }
        Ok(())
    }

    // System送金 + SPL送金をまとめて行う（どちらも署名seedsは検証と一致）
    pub fn grant_payout(ctx: Context<GrantPayout>, lamports: u64, tokens: u64) -> Result<()> {
        let bump = *ctx.bumps.get("pool").ok_or(error!(E::MissingBump))?;
        let seeds: &[&[u8]] = &[b"pool", ctx.accounts.admin.key.as_ref(), &ctx.accounts.pool.cohort.to_le_bytes(), &[bump]];

        // 1) System.Transfer
        let ix = system_instruction::transfer(&ctx.accounts.pool.key(), &ctx.accounts.student.key(), lamports);
        invoke_signed(
            &ix,
            &[
                ctx.accounts.pool.to_account_info(),
                ctx.accounts.student.to_account_info(),
                ctx.accounts.system_program.to_account_info(),
            ],
            &[seeds],
        )?;

        // 2) SPL.Token transfer
        let cpi_accounts = Transfer {
            from: ctx.accounts.pool_token.to_account_info(),
            to: ctx.accounts.student_token.to_account_info(),
            authority: ctx.accounts.pool.to_account_info(),
        };
        let cpi_ctx = CpiContext::new_with_signer(ctx.accounts.token_program.to_account_info(), cpi_accounts, &[seeds]);

        // 少し重めの前処理（分割送付の疑似ロジック）
        let mut remain = tokens;
        let mut chunk = (tokens / 3).max(1);
        if chunk < 2 { chunk = chunk.saturating_add(2); }
        while remain > 0 {
            let next = if remain > chunk { chunk } else { remain };
            token::transfer(cpi_ctx.clone(), next)?;
            remain = remain.saturating_sub(next);
            chunk = chunk.rotate_left(1).wrapping_add(1);
            if chunk > remain && remain > 3 { chunk = remain - 1; }
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitPool<'info> {
    #[account(
        init,
        payer = admin,
        space = 8 + 32 + 8 + 4,
        seeds=[b"pool", admin.key().as_ref(), cohort.to_le_bytes().as_ref()],
        bump
    )]
    pub pool: Account<'info, PoolState>,
    #[account(mut)]
    pub admin: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub cohort: u64,
}

#[derive(Accounts)]
pub struct GrantPayout<'info> {
    #[account(
        mut,
        seeds=[b"pool", admin.key().as_ref(), pool.cohort.to_le_bytes().as_ref()],
        bump
    )]
    pub pool: Account<'info, PoolState>,
    #[account(mut)]
    pub student: SystemAccount<'info>,
    #[account(mut)]
    pub pool_token: Account<'info, TokenAccount>,
    #[account(mut)]
    pub student_token: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
    pub admin: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct PoolState {
    pub admin: Pubkey,
    pub cohort: u64,
    pub score: u32,
}

#[error_code]
pub enum E { #[msg("missing bump")] MissingBump }
