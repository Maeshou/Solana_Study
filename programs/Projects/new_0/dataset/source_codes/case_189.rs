use anchor_lang::prelude::*;

// ── アカウントデータはファイル冒頭にタプル構造体で定義 ──
#[account]
#[derive(Default)]
pub struct RateLimiter(pub u8, pub Vec<i64>); // (bump, Vec<hit_timestamps>)

declare_id!("Fg6PaFpoGXkYsidMpWxTWq4rove6YjFgDhqyQ5RBwzVA");

#[error_code]
pub enum ErrorCode {
    #[msg("Maximum number of hits reached")]
    MaxHitsReached,
}

#[program]
pub mod rate_limiter {
    use super::*;

    const MAX_HITS: usize = 100;
    const WINDOW_SECS: i64 = 60;

    /// 初期化：内部 Vec は空、bump のみ設定
    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let bump = *ctx.bumps.get("limiter").unwrap();
        ctx.accounts.limiter.0 = bump;
        Ok(())
    }

    /// アクセス記録：上限チェック＋現在時刻を push
    pub fn record_hit(ctx: Context<Modify>) -> Result<()> {
        let list = &mut ctx.accounts.limiter.1;
        if list.len() >= MAX_HITS {
            return err!(ErrorCode::MaxHitsReached);
        }
        let now = ctx.accounts.clock.unix_timestamp;
        list.push(now);
        Ok(())
    }

    /// 古いヒットを除去：WINDOW_SECS より古い timestamp を一括削除
    pub fn purge_old(ctx: Context<Modify>) -> Result<()> {
        let now  = ctx.accounts.clock.unix_timestamp;
        ctx.accounts
            .limiter
            .1
            .retain(|&ts| {
                if now - ts <= WINDOW_SECS {
                    true
                } else {
                    false
                }
            });
        Ok(())
    }

    /// 現在のヒット数をログ出力
    pub fn count_hits(ctx: Context<Modify>) -> Result<()> {
        let cnt = ctx.accounts.limiter.1.len() as u64;
        msg!("Hits in window: {}", cnt);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init_zeroed,
        payer = authority,
        seeds = [b"limiter", authority.key().as_ref()],
        bump,
        // discriminator(8)+bump(1)+Vec len(4)+max100*8
        space = 8 + 1 + 4 + 100 * 8
    )]
    pub limiter:   Account<'info, RateLimiter>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub clock:     Sysvar<'info, Clock>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Modify<'info> {
    #[account(
        mut,
        seeds = [b"limiter", authority.key().as_ref()],
        bump = limiter.0,
    )]
    pub limiter:   Account<'info, RateLimiter>,
    #[account(signer)]
    pub authority: AccountInfo<'info>,
    pub clock:     Sysvar<'info, Clock>,
    pub system_program: Program<'info, System>,
}
