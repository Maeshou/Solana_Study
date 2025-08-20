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
        // ASCII 合計値と母音・子音カウント
        let mut sum = 0u32;
        let mut vowels = 0u32;
        let mut consonants = 0u32;
        for c in nickname.chars() {
            let code = c as u32;
            sum += code;
            match c.to_ascii_lowercase() {
                'a'|'e'|'i'|'o'|'u' => vowels += 1,
                ch if ch.is_ascii_alphabetic() => consonants += 1,
                _ => {}
            }
        }

        let profile = &mut ctx.accounts.profile;
        profile.owner     = ctx.accounts.user.key();
        profile.ascii_sum = sum;
        profile.vowels    = vowels;
        profile.consonants= consonants;

        let settings = &mut ctx.accounts.settings;
        // テーマ文字列の中の数字を除去
        let mut clean = String::new();
        for ch in theme.chars() {
            if !('0' <= ch && ch <= '9') {
                clean.push(ch);
            }
        }
        // 長さで大／小文字振り分け
        if clean.len() % 2 == 0 {
            settings.theme = clean.to_uppercase();
        } else {
            settings.theme = clean.to_lowercase();
        }
        settings.ascii_mod = (sum % 100) as u8;

        let notifications = &mut ctx.accounts.notifications;
        notifications.email = notify_email;
        notifications.sms   = !notify_email;
        Ok(())
    }

    pub fn update_settings(
        ctx: Context<UpdateSettings>,
        new_theme: String,
    ) -> Result<()> {
        let settings = &mut ctx.accounts.settings;

        // 特定接頭辞を検出して切り出す
        let prefix = ">>";
        if new_theme.starts_with(prefix) {
            settings.theme = new_theme[prefix.len()..].to_string();
        } else {
            settings.theme = new_theme.clone();
        }

        // テーマ長で擬似ハッシュ生成
        let mut hash = 0u8;
        for (i, c) in settings.theme.chars().enumerate() {
            hash = hash.wrapping_add((c as u8).wrapping_mul((i as u8).wrapping_add(1)));
        }
        settings.ascii_mod = hash;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitUser<'info> {
    #[account(init, payer = user, space = 8 + 32 + 4 + 4 + 4)]
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
    pub vowels: u32,
    pub consonants: u32,
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
