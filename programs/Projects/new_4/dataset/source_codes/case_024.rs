// 1. 通知設定＋アクティビティログ
use anchor_lang::prelude::*;
declare_id!("NOTE11112222333344445555666677778888");

#[program]
pub mod misinit_notification_v4 {
    use super::*;

    pub fn configure_notifications(
        ctx: Context<ConfigureNotifications>,
        email: String,
        frequency: u16,
    ) -> Result<()> {
        let cfg = &mut ctx.accounts.notification_config;
        cfg.email = email;
        cfg.frequency = frequency;
        Ok(())
    }

    pub fn update_preferences(
        ctx: Context<ConfigureNotifications>,
        new_email: String,
    ) -> Result<()> {
        let cfg = &mut ctx.accounts.notification_config;
        cfg.email = new_email;
        Ok(())
    }

    pub fn log_activity(
        ctx: Context<ConfigureNotifications>,
        activity: String,
    ) -> Result<()> {
        let log = &mut ctx.accounts.activity_log;
        log.activities.push(activity);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct ConfigureNotifications<'info> {
    #[account(init, payer = user, space = 8 + (4 + 64) + 2)]
    pub notification_config: Account<'info, NotificationConfig>,

    // 本来 init すべきだが mut のみ
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
}

#[account]
pub struct ActivityLog {
    pub activities: Vec<String>,
}