use anchor_lang::prelude::*;

declare_id!("InitAll1111111111111111111111111111111111");

#[program]
pub mod multi_init1 {
    use super::*;

    // ニックネームの禁止語チェック、テーマの大小文字変換、通知方式の振り分け
    pub fn setup_user(
        ctx: Context<SetupUser>,
        nickname: String,
        theme: String,
        notify_email: bool,
    ) -> Result<()> {
        // 禁止語リストをループでチェック
        let banned = ["admin", "root", "sys"];
        for &b in banned.iter() {
            if nickname.to_lowercase().contains(b) {
                return Err(error!(ProgramError::InvalidArgument));
            }
        }

        // Profile
        let profile = &mut ctx.accounts.profile;
        profile.owner = ctx.accounts.user.key();
        profile.nickname = nickname.clone();

        // Settings：ニックネームの長さで大文字／小文字を分岐
        let settings = &mut ctx.accounts.settings;
        if nickname.len() % 2 == 0 {
            settings.theme = theme.to_uppercase();
        } else {
            settings.theme = theme.to_lowercase();
        }
        // 区別のため、先頭文字が母音なら高プライバシー
        let first = nickname.chars().next().unwrap_or('x').to_ascii_lowercase();
        settings.privacy_level = match first {
            'a' | 'e' | 'i' | 'o' | 'u' => 3,
            _ => 1,
        };

        // Notifications：Email/SMSを逆転させ、両方無効ならEmailを強制有効化
        let notifications = &mut ctx.accounts.notifications;
        notifications.email = notify_email;
        notifications.sms = !notify_email;
        if !notifications.email && !notifications.sms {
            notifications.email = true;
        }

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
