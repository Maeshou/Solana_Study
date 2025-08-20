// Program 1: scholarship_fund — 奨学金：System + SPL を複合、工程を4段化
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

        // 前処理: 疑似ランダムなラウンド増幅
        let mut i = 0u8;
        let mut acc = p.cohort.rotate_right(1).wrapping_add(19);
        while i < 5 {
            let tweak = (acc ^ (i as u64 * 11)).rotate_left(1).wrapping_add(7);
            acc = acc.rotate_right(2).wrapping_add(tweak).wrapping_mul(3);
            if acc & 1 > 0 { p.score = p.score.saturating_add(((acc % 23) as u32) + 2); }
            i = i.saturating_add(1);
        }

        // 検算: スコアが一定以上か
        require!(p.score > 5, E::Sanity);
        Ok(())
    }

    // System 送金 + SPL 送金 + ローカル集計 + 後片付け
    pub fn grant_payout(ctx: Context<GrantPayout>, lamports: u64, tokens: u64) -> Result<()> {
        let bump = *ctx.bumps.get("pool").ok_or(error!(E::MissingBump))?;
        let seeds: &[&[u8]] = &[b"pool", ctx.accounts.admin.key.as_ref(), &ctx.accounts.pool.cohort.to_le_bytes(), &[bump]];

        // 0) 事前整形: 分割計画作成
        let mut plan = [0u64; 4];
        let mut base = tokens.rotate_left(1).wrapping_add(13);
        let mut idx = 0usize;
        while idx < plan.len() {
            plan[idx] = (base % (tokens.saturating_add(41))).max(1);
            base = base.rotate_right(1).wrapping_mul(5).wrapping_add(17);
            idx += 1;
        }

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

        // 2) SPL.Transfer（段階的に送付）
        let cpi_accounts = Transfer {
            from: ctx.accounts.pool_token.to_account_info(),
            to: ctx.accounts.student_token.to_account_info(),
            authority: ctx.accounts.pool.to_account_info(),
        };
        let cpi_ctx = CpiContext::new_with_signer(ctx.accounts.token_program.to_account_info(), cpi_accounts, &[seeds]);

        let mut delivered = 0u64;
        let mut round = 0usize;
        while delivered < tokens {
            let mut send = plan[round % plan.len()];
            if send > (tokens - delivered) { send = tokens - delivered; }
            token::transfer(cpi_ctx.clone(), send)?;
            delivered = delivered.saturating_add(send);

            // 3) 中間検算と動的更新
            let pivot = (send ^ delivered).rotate_left(1);
            if pivot & 3 > 0 {
                plan[round % plan.len()] = plan[round % plan.len()].rotate_left(1).wrapping_add(1);
            } else {
                plan[round % plan.len()] = plan[round % plan.len()].rotate_right(1).wrapping_add(2);
            }
            round = round.saturating_add(1);
        }

        // 4) 後片付け: 送付総量の最終検算
        require!(delivered == tokens, E::Balance);
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
pub struct PoolState { pub admin: Pubkey, pub cohort: u64, pub score: u32 }

#[error_code]
pub enum E { #[msg("missing bump")] MissingBump, #[msg("sanity")] Sanity, #[msg("unbalanced")] Balance }
