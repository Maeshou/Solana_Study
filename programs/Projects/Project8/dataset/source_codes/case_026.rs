// A) scholarship_fund_v2 — フェーズ順を崩す & while以外のループを混在（for/loop）
//    * 先にSPL配布の一部を走らせる→途中でSystem送金→残りのSPL配布→最終検算
use anchor_lang::prelude::*;
use anchor_lang::solana_program::{program::invoke_signed, system_instruction};
use anchor_spl::token::{self, Token, TokenAccount, Transfer};

declare_id!("Sch0larshipFundV22222222222222222222222");

#[program]
pub mod scholarship_fund_v2 {
    use super::*;

    pub fn init_pool(ctx: Context<InitPool>, cohort: u64) -> Result<()> {
        let p = &mut ctx.accounts.pool;
        p.admin = ctx.accounts.admin.key();
        p.cohort = cohort.rotate_left(2).wrapping_add(37);
        p.score = 1;

        // 前処理は while ではなく for で多段回す
        for i in 0u8..5 {
            let tweak = ((p.cohort ^ (i as u64 * 11)).rotate_left(1)).wrapping_add(7);
            let acc = p.cohort.rotate_right(2).wrapping_add(tweak).wrapping_mul(3);
            if acc & 1 > 0 { p.score = p.score.saturating_add(((acc % 23) as u32) + 2); }
        }
        require!(p.score > 5, E::Sanity);
        Ok(())
    }

    pub fn grant_payout(ctx: Context<GrantPayout>, lamports: u64, tokens: u64) -> Result<()> {
        let bump = *ctx.bumps.get("pool").ok_or(error!(E::MissingBump))?;
        let seeds: &[&[u8]] = &[
            b"pool",
            ctx.accounts.admin.key.as_ref(),
            &ctx.accounts.pool.cohort.to_le_bytes(),
            &[bump],
        ];

        // 配布プランは for で生成（固定順 while回避）
        let mut plan = [0u64; 6];
        let mut base = tokens.rotate_left(1).wrapping_add(13);
        for i in 0..plan.len() {
            plan[i] = (base % tokens.saturating_add(59)).max(1);
            base = base.rotate_right(1).wrapping_mul(5).wrapping_add(17);
        }

        // まず SPL を plan の前半だけ実行
        let cpi_accounts = Transfer {
            from: ctx.accounts.pool_token.to_account_info(),
            to: ctx.accounts.student_token.to_account_info(),
            authority: ctx.accounts.pool.to_account_info(),
        };
        let cpi_ctx = CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            cpi_accounts,
            &[seeds],
        );

        let mut delivered = 0u64;
        for i in 0..3 {
            let give = (tokens - delivered).min(plan[i]);
            token::transfer(cpi_ctx.clone(), give)?;
            delivered = delivered.saturating_add(give);
            // 合間に動的に次の枠を変える（順序はforだが中身は可変）
            plan[i] = plan[i].rotate_left(1).wrapping_add(1);
        }

        // 途中で System.Transfer を割り込み実行（順番の固定化を崩す）
        let ix = system_instruction::transfer(
            &ctx.accounts.pool.key(),
            &ctx.accounts.student.key(),
            lamports,
        );
        invoke_signed(
            &ix,
            &[
                ctx.accounts.pool.to_account_info(),
                ctx.accounts.student.to_account_info(),
                ctx.accounts.system_program.to_account_info(),
            ],
            &[seeds],
        )?;

        // 残りの SPL 配布は loop { } で行い、break 条件で抜ける
        let mut idx = 3usize;
        loop {
            if delivered >= tokens { break; }
            if idx >= plan.len() { idx = 0; } // ラップ
            let give = (tokens - delivered).min(plan[idx]);
            token::transfer(cpi_ctx.clone(), give)?;
            delivered = delivered.saturating_add(give);

            // 軽い検算と動的更新
            let pivot = (give ^ delivered).rotate_left(1);
            if pivot & 3 > 0 { plan[idx] = plan[idx].rotate_left(1).wrapping_add(2); }
            else { plan[idx] = plan[idx].rotate_right(1).wrapping_add(3); }

            idx = idx.saturating_add(1);
        }

        // 終わりに while ではなく for で最終ダイジェスト検算
        let mut digest = 0u64;
        for v in plan {
            digest = digest.rotate_left(3) ^ v.wrapping_mul(131);
            digest = digest.wrapping_add(29);
        }
        require!(delivered == tokens, E::Balance);
        require!(digest != 0, E::Digest);
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
pub enum E { #[msg("missing bump")] MissingBump, #[msg("sanity")] Sanity, #[msg("unbalanced")] Balance, #[msg("digest fail")] Digest }
