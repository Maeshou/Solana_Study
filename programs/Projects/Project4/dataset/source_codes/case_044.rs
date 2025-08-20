use anchor_lang::prelude::*;

declare_id!("SafeMulti1111111111111111111111111111111111");

#[program]
pub mod safe_profile {
    use super::*;

    // profile, settings, notifications をすべて初期化
    pub fn setup_user(
        ctx: Context<SetupUser>,
        nickname: String,
        theme: String,
        notify_email: bool,
    ) -> Result<()> {
        // Profile
        let profile = &mut ctx.accounts.profile;
        profile.owner = ctx.accounts.user.key();
        profile.nickname = nickname.clone();

        // Settings：ニックネーム長で分岐
        let settings = &mut ctx.accounts.settings;
        if nickname.len() % 2 == 0 {
            settings.theme = theme.to_uppercase();
        } else {
            settings.theme = theme.to_lowercase();
        }
        settings.privacy_level = (nickname.len() % 4) as u8;

        // Notifications：メール／SMS振り分け
        let notifications = &mut ctx.accounts.notifications;
        notifications.email = notify_email;
        notifications.sms   = !notify_email;

        Ok(())
    }

    // settings だけを mut 更新
    pub fn update_settings(
        ctx: Context<UpdateSettings>,
        new_theme: String,
        new_privacy: u8,
    ) -> Result<()> {
        let settings = &mut ctx.accounts.settings;
        settings.theme = new_theme;
        settings.privacy_level = new_privacy;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct SetupUser<'info> {
    #[account(init, payer = user, space = 8 + 32 + 64)]
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
    #[account(mut)]
    pub settings: Account<'info, SettingsData>,
}

#[account]
pub struct ProfileData {
    pub owner: Pubkey,
    pub nickname: String,
}

#[account]
pub struct SettingsData {
    pub theme: String,
    pub privacy_level: u8,
}

#[account]
pub struct NotificationsData {
    pub email: bool,
    pub sms: bool,
}
