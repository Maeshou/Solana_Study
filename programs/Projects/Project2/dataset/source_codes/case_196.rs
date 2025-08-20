use anchor_lang::prelude::*;

// ── アカウントデータはファイル冒頭にタプル構造体で定義 ──
#[account]
#[derive(Default)]
pub struct PomodoroTracker(pub u8, pub Vec<(i64, u64)>); // (bump, Vec<(start_ts, duration_secs)>)

declare_id!("Fg6PaFpoGXkYsidMpWxTWq4rove6YjFgDhqyQ5RBwzVH");

#[error_code]
pub enum ErrorCode {
    #[msg("Maximum number of sessions reached")]
    MaxSessionsReached,
    #[msg("No active session to complete")]
    NoActiveSession,
}

#[program]
pub mod pomodoro_tracker {
    use super::*;

    const MAX_SESSIONS: usize = 5;

    /// 初期化：内部 Vec は空、bump のみ設定
    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let b = *ctx.bumps.get("tracker").unwrap();
        ctx.accounts.tracker.0 = b;
        Ok(())
    }

    /// Pomodoro 開始：件数制限チェック＋開始時刻のみ追加（duration は 0 で初期化）
    pub fn start_session(ctx: Context<Modify>) -> Result<()> {
        let list = &mut ctx.accounts.tracker.1;
        if list.len() >= MAX_SESSIONS {
            return err!(ErrorCode::MaxSessionsReached);
        }
        let now = ctx.accounts.clock.unix_timestamp;
        list.push((now, 0));
        Ok(())
    }

    /// Pomodoro 完了：最後のセッションの duration を設定
    pub fn complete_session(ctx: Context<Modify>, duration_secs: u64) -> Result<()> {
        let list = &mut ctx.accounts.tracker.1;
        if let Some(last) = list.last_mut() {
            last.1 = duration_secs;
            Ok(())
        } else {
            err!(ErrorCode::NoActiveSession)
        }
    }

    /// 古いセッション削除：start_ts が cutoff より前のものを除去
    pub fn purge_old(ctx: Context<Modify>, cutoff: i64) -> Result<()> {
        let list = &mut ctx.accounts.tracker.1;
        list.retain(|&(start, _)| {
            if start < cutoff {
                false
            } else {
                true
            }
        });
        Ok(())
    }

    /// 完了済みセッション数をログ出力
    pub fn count_completed(ctx: Context<Modify>) -> Result<()> {
        let list = &ctx.accounts.tracker.1;
        let mut cnt = 0u64;
        for &(_, duration) in list.iter() {
            if duration > 0 {
                cnt = cnt.wrapping_add(1);
            }
        }
        msg!("Completed sessions: {}", cnt);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init_zeroed,
        payer = authority,
        seeds = [b"tracker", authority.key().as_ref()],
        bump,
        // discriminator(8) + bump(1) + Vec len(4) + max5*(8+8)
        space = 8 + 1 + 4 + 5 * (8 + 8)
    )]
    pub tracker:   Account<'info, PomodoroTracker>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub clock:     Sysvar<'info, Clock>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Modify<'info> {
    #[account(
        mut,
        seeds = [b"tracker", authority.key().as_ref()],
        bump = tracker.0,
    )]
    pub tracker:   Account<'info, PomodoroTracker>,
    #[account(signer)]
    pub authority: AccountInfo<'info>,
    pub clock:     Sysvar<'info, Clock>,
    pub system_program: Program<'info, System>,
}
