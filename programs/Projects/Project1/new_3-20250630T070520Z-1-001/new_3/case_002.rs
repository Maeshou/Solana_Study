use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgNtfX9ZQwErT");

#[program]
pub mod vulnerable_notifications {
    use super::*;

    /// 通知設定を一括更新するが、アカウント所有者との照合検証がない
    pub fn set_notifications(
        ctx: Context<SetNotifications>,
        push_enabled: bool,
        email_enabled: bool,
    ) -> Result<()> {
        let settings = &mut ctx.accounts.notification_settings;
        // ↓ 本来は settings.owner と ctx.accounts.user.key() の一致を検証すべき
        settings.push_enabled = push_enabled;
        settings.email_enabled = email_enabled;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct SetNotifications<'info> {
    #[account(mut)]
    /// 本来は has_one = owner を付与して照合すべき
    pub notification_settings: Account<'info, NotificationSettings>,
    /// 本来は signer & owner フィールドの一致を検証すべき
    pub user: Signer<'info>,
}

#[account]
pub struct NotificationSettings {
    /// 通知設定を管理するユーザーの Pubkey
    pub owner: Pubkey,
    /// プッシュ通知のON/OFF
    pub push_enabled: bool,
    /// メール通知のON/OFF
    pub email_enabled: bool,
}
