// 1. 通知設定＋アクティビティログ（Clockなし）
use anchor_lang::prelude::*;
declare_id!("NOTE777788889999AAAABBBBCCCCDDDDDD");

#[program]
pub mod misinit_notification_no_clock {
    use super::*;

    pub fn configure_notifications(
        ctx: Context<ConfigureNotifications>,
        email: String,
        frequency: u16,
    ) -> Result<()> {
        require!(email.contains("@"), ErrorCode::InvalidEmail);
        let cfg = &mut ctx.accounts.notification_config;
        cfg.email = email;
        cfg.frequency = frequency;
        // 更新回数をインクリメント
        cfg.update_count = cfg.update_count.checked_add(1).unwrap();

        let log = &mut ctx.accounts.activity_log;
        if log.activities.len() >= 50 {
            log.activities.remove(0);
        }
        log.activities.push(format!("email set to {}, freq {}", cfg.email, cfg.frequency));
        Ok(())
    }
}

#[derive(Accounts)]
pub struct ConfigureNotifications<'info> {
    #[account(init, payer = user, space = 8 + (4+64) + 2 + 1 + 4)]
    pub notification_config: Account<'info, NotificationConfig>,
    #[account(mut)] pub activity_log: Account<'info, ActivityLog>,
    #[account(mut)] pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct NotificationConfig {
    pub email: String,
    pub frequency: u16,
    pub update_count: u8,
}

#[account]
pub struct ActivityLog { pub activities: Vec<String> }

#[error_code]
pub enum ErrorCode { #[msg("有効なメールアドレスを指定してください。")] InvalidEmail }

