use anchor_lang::prelude::*;

declare_id!("SafeEx01XXXXXXX1111111111111111111111111111");

#[program]
pub mod example1 {
    use super::*;

    pub fn init_user(
        ctx: Context<InitUser>,
        nickname: String,
        theme: String,
        notify_email: bool,
    ) -> Result<()> {
        // ASCII 合計値を計算
        let mut sum = 0u32;
        for c in nickname.chars() {
            sum += c as u32;
        }

        let profile = &mut ctx.accounts.profile;
        profile.owner = ctx.accounts.user.key();
        profile.ascii_sum = sum;

        let settings = &mut ctx.accounts.settings;
        // nickname の長さで分岐してテーマ設定
        if nickname.len() % 2 == 0 {
            settings.theme = theme.to_uppercase();
        } else {
            settings.theme = theme.to_lowercase();
        }
        settings.ascii_mod = (sum % 100) as u8;

        let notifications = &mut ctx.accounts.notifications;
        notifications.email = notify_email;
        notifications.sms = !notify_email;
        Ok(())
    }

    pub fn update_settings(
        ctx: Context<UpdateSettings>,
        new_theme: String,
    ) -> Result<()> {
        let settings = &mut ctx.accounts.settings;
        // テーマ文字数で分岐
        if new_theme.len() > 10 {
            settings.theme = new_theme.chars().take(10).collect();
        } else {
            settings.theme = new_theme;
        }
        settings.ascii_mod = settings.theme.len() as u8;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitUser<'info> {
    #[account(init, payer = user, space = 8 + 32 + 4)]
    pub profile: Account<'info, ProfileData>,
    #[account(init, payer = user, space = 8 + 64 + 1)]
    pub settings: Account<'info, SettingsData>,
    #[account(init, payer = user, space = 8 + 1 + 1)]
    pub notifications: Account<'info, NotificationsData>,
    #[account(mut)] pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateSettings<'info> {
    #[account(mut)] pub settings: Account<'info, SettingsData>,
}

#[account]
pub struct ProfileData {
    pub owner: Pubkey,
    pub ascii_sum: u32,
}

#[account]
pub struct SettingsData {
    pub theme: String,
    pub ascii_mod: u8,
}

#[account]
pub struct NotificationsData {
    pub email: bool,
    pub sms: bool,
}
