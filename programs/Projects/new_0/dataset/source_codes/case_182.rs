use anchor_lang::prelude::*;

// ── アカウントデータはファイル冒頭にタプル構造体で定義 ──
#[account]
#[derive(Default)]
pub struct NotificationManager(pub u8, pub Vec<(i64, String)>);

declare_id!("Fg6PaFpoGXkYsidMpWxTWq4rove6YjFgDhqyQ5RBwzV4");

#[program]
pub mod notification_manager {
    use super::*;

    /// 管理アカウント初期化：内部 Vec は空のまま、bump のみ設定
    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let bump = *ctx.bumps.get("manager").unwrap();
        ctx.accounts.manager.0 = bump;
        Ok(())
    }

    /// 通知スケジュール：unlock_ts（UNIX時刻）とメッセージを追加
    pub fn schedule(
        ctx: Context<Modify>,
        unlock_ts: i64,
        message: String,
    ) -> Result<()> {
        let list = &mut ctx.accounts.manager.1;
        list.push((unlock_ts, message));
        Ok(())
    }

    /// 期限到来通知：now より古いエントリをログ出力し、一括削除
    pub fn send_due(ctx: Context<Modify>) -> Result<()> {
        let list = &mut ctx.accounts.manager.1;
        let now  = ctx.accounts.clock.unix_timestamp;

        // 古いものを順にログ出力
        for &(ts, ref msg) in list.iter() {
            if ts < now {
                msg!("Notification at {}: {}", ts, msg);
            }
        }

        // now より大きいタイムスタンプだけ残す
        list.retain(|&(ts, _)| ts > now);
        Ok(())
    }

    /// 全通知クリア
    pub fn clear_all(ctx: Context<Modify>) -> Result<()> {
        ctx.accounts.manager.1.clear();
        Ok(())
    }
}

// ── Context 定義は末尾に配置 ──
#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init_zeroed,
        payer = authority,
        seeds = [b"manager", authority.key().as_ref()],
        bump,
        // discriminator(8) + bump(1)
        // + Vec<(i64,String)> (max 10 件: 4 + 10*(8+4+200))
        space = 8 + 1 + 4 + 10 * (8 + 4 + 200)
    )]
    pub manager:   Account<'info, NotificationManager>,

    /// 管理者（署名必須）
    #[account(mut)]
    pub authority: Signer<'info>,

    /// 時刻取得用
    pub clock:     Sysvar<'info, Clock>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Modify<'info> {
    #[account(
        mut,
        seeds = [b"manager", authority.key().as_ref()],
        bump = manager.0,
    )]
    pub manager:   Account<'info, NotificationManager>,

    /// 操作を行うユーザー（署名必須）
    #[account(signer)]
    pub authority: AccountInfo<'info>,

    /// 時刻取得用
    pub clock:     Sysvar<'info, Clock>,

    pub system_program: Program<'info, System>,
}
