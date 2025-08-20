use anchor_lang::prelude::*;

declare_id!("OwnChkEXT00000000000000000000000000000001");

#[program]
pub mod user_settings_ext {
    pub fn update_settings(
        ctx: Context<UpdateSettings>,
        theme: String,
        notifications: bool,
        language: String,
    ) -> Result<()> {
        let cfg = &mut ctx.accounts.settings;
        // 所有者検証済み (has_one = owner)
        cfg.theme          = theme.clone();
        cfg.notifications  = notifications;
        cfg.language       = language.clone();
        cfg.version        = cfg.version.saturating_add(1);
        cfg.last_updated   = Clock::get()?.unix_timestamp;

        // 任意のキー設定もマップで保持
        cfg.preferences.insert("theme".into(), cfg.version.to_string());
        cfg.preferences.insert("lang".into(), language);

        // audit_buf は unchecked
        let buf = &mut ctx.accounts.audit_buf.data.borrow_mut();
        buf.extend_from_slice(b"settings updated;");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct UpdateSettings<'info> {
    #[account(mut, has_one = owner)]
    pub settings: Account<'info, UserSettings>,
    pub owner: Signer<'info>,
    /// CHECK: 監査用バッファ。所有者検証なし
    #[account(mut)]
    pub audit_buf: AccountInfo<'info>,
    pub clock: Sysvar<'info, Clock>,
}

#[account]
pub struct UserSettings {
    pub owner: Pubkey,
    pub theme: String,
    pub notifications: bool,
    pub language: String,
    pub version: u64,
    pub last_updated: i64,
    pub preferences: std::collections::BTreeMap<String, String>,
}
