// コード例をより詳細にし、各モジュールで状態管理や履歴操作などのロジックを追加

// 1. 通知設定＋アクティビティログ
use anchor_lang::prelude::*;
declare_id!("NOTE11112222333344445555666677778888");

#[program]
pub mod misinit_notification_v4 {
    use super::*;

    // メール通知の設定、新規作成
    pub fn configure_notifications(
        ctx: Context<ConfigureNotifications>,
        email: String,
        frequency: u16,
    ) -> Result<()> {
        let cfg = &mut ctx.accounts.notification_config;
        // メールフォーマットの最低限チェック
        require!(email.contains("@"), ErrorCode::InvalidEmail);
        cfg.email = email;
        cfg.frequency = frequency;
        cfg.updated_at = Clock::get()?.unix_timestamp;
        // 前回の履歴を保持
        let log = &mut ctx.accounts.activity_log;
        log.activities.push(format!("configured: {}hz at {}", frequency, cfg.updated_at));
        Ok(())
    }

    // 設定の一部を更新
    pub fn update_preferences(
        ctx: Context<ConfigureNotifications>,
        new_email: Option<String>,
        new_frequency: Option<u16>,
    ) -> Result<()> {
        let cfg = &mut ctx.accounts.notification_config;
        if let Some(e) = new_email {
            require!(e.contains("@"), ErrorCode::InvalidEmail);
            cfg.email = e;
        }
        if let Some(f) = new_frequency {
            cfg.frequency = f;
        }
        cfg.updated_at = Clock::get()?.unix_timestamp;
        // ログも記録
        let log = &mut ctx.accounts.activity_log;
        log.activities.push(format!("updated at {}", cfg.updated_at));
        Ok(())
    }

    // 活動を詳細に記録
    pub fn log_activity(
        ctx: Context<ConfigureNotifications>,
        activity: String,
    ) -> Result<()> {
        let log = &mut ctx.accounts.activity_log;
        // 最大100件に制限
        if log.activities.len() >= 100 {
            log.activities.remove(0);
        }
        log.activities.push(activity);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct ConfigureNotifications<'info> {
    #[account(init, payer = user, space = 8 + (4 + 128) + 2 + 8)]
    pub notification_config: Account<'info, NotificationConfig>,

    #[account(mut)]
    pub activity_log: Account<'info, ActivityLog>,

    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct NotificationConfig {
    pub email: String,
    pub frequency: u16,
    pub updated_at: i64,
}

#[account]
pub struct ActivityLog {
    pub activities: Vec<String>,
}

#[error_code]
pub enum ErrorCode {
    #[msg("有効なメールアドレスを指定してください。")]
    InvalidEmail,
}