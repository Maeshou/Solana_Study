use anchor_lang::prelude::*;

// ── アカウントデータをファイル冒頭に定義 ──
#[account]
#[derive(Default)]
pub struct Attendance {
    pub bump:            u8,            // PDA bump
    pub attendees:       Vec<Pubkey>,   // 参加者リスト
    pub threshold:       u8,            // 自動クローズする参加者数
    pub closed:          bool,          // クローズ済みフラグ
    pub last_update_ts:  i64,           // 最終更新タイムスタンプ
}

declare_id!("Fg6PaFpoGXkYsidMpWxTWq4rove6YjFgDhqyQ5RBwzUQ");

#[program]
pub mod attendance_tracker {
    use super::*;

    /// 出席トラッカー初期化：thresholdだけ設定し、残りはデフォルト
    pub fn initialize_attendance(
        ctx: Context<InitializeAttendance>,
        threshold: u8,
    ) -> Result<()> {
        let atd = &mut ctx.accounts.attendance;
        let now = ctx.accounts.clock.unix_timestamp;
        *atd = Attendance {
            bump:            *ctx.bumps.get("attendance").unwrap(),
            threshold,
            last_update_ts:  now,
            ..Default::default()
        };
        Ok(())
    }

    /// チェックイン：まだクローズしていなければ参加を記録し、閾値到達で自動クローズ
    pub fn check_in(
        ctx: Context<ModifyAttendance>,
    ) -> Result<()> {
        let atd = &mut ctx.accounts.attendance;
        let now = ctx.accounts.clock.unix_timestamp;

        // まだオープンなら
        if !atd.closed {
            atd.attendees.push(ctx.accounts.user.key());
            // 閾値到達で閉鎖フラグを立てる
            if atd.attendees.len() as u8 >= atd.threshold {
                atd.closed = true;
            }
        }

        // 最終更新時刻を常に更新
        atd.last_update_ts = now;
        Ok(())
    }

    /// 再オープン：閉鎖を解除し、時刻を更新
    pub fn reopen(
        ctx: Context<ModifyAttendance>,
    ) -> Result<()> {
        let atd = &mut ctx.accounts.attendance;
        atd.closed          = false;
        atd.last_update_ts  = ctx.accounts.clock.unix_timestamp;
        Ok(())
    }
}

// ── コンテキスト定義は末尾に配置 ──
#[derive(Accounts)]
#[instruction(threshold: u8)]
pub struct InitializeAttendance<'info> {
    #[account(
        init_zeroed,
        payer = authority,
        seeds = [b"attendance", authority.key().as_ref()],
        bump,
        // discriminator(8) + bump(1) + Vec<Pubkey> (max10*32+4) + threshold(1) + closed(1) + last_update_ts(8)
        space = 8 + 1 + (4 + 10*32) + 1 + 1 + 8
    )]
    pub attendance: Account<'info, Attendance>,

    /// 管理者（署名必須）
    #[account(mut)]
    pub authority: Signer<'info>,

    /// 時刻取得用
    pub clock: Sysvar<'info, Clock>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ModifyAttendance<'info> {
    #[account(
        mut,
        seeds = [b"attendance", authority.key().as_ref()],
        bump = attendance.bump,
    )]
    pub attendance: Account<'info, Attendance>,

    /// チェックイン／再オープンを行うユーザー（署名必須）
    #[account(signer)]
    pub authority: AccountInfo<'info>,

    /// チェックイン対象のユーザー（キーのみ使用）
    pub user: AccountInfo<'info>,

    /// 時刻取得用
    pub clock: Sysvar<'info, Clock>,

    pub system_program: Program<'info, System>,
}
