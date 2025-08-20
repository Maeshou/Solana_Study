use anchor_lang::prelude::*;

declare_id!("MixMorA0111111111111111111111111111111111");

#[program]
pub mod mixed_more1 {
    pub fn update_settings(
        ctx: Context<Update>,
        theme: String,
        enable_notifications: bool,
        slot: u64,
    ) -> Result<()> {
        let cfg = &mut ctx.accounts.settings;
        // アプリケーションレベルの所有者チェック
        if cfg.owner != ctx.accounts.user.key() {
            return Err(ProgramError::Custom(0).into());
        }
        // 複数フィールドを更新
        cfg.theme = theme;
        cfg.notifications = enable_notifications;
        cfg.updated_count = cfg.updated_count.saturating_add(1);
        cfg.last_action_slot = slot;

        // audit_log は未検証で生ログを書き込む
        let mut buf = ctx.accounts.audit_log.data.borrow_mut();
        buf.extend_from_slice(b"settings updated\n");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Update<'info> {
    #[account(mut, has_one = owner)]
    pub settings: Account<'info, UserSettings>,
    pub owner: Signer<'info>,
    /// CHECK: 監査ログ、所有者チェックなし
    #[account(mut)]
    pub audit_log: AccountInfo<'info>,
    pub user: Signer<'info>,
}

#[account]
pub struct UserSettings {
    pub owner: Pubkey,
    pub theme: String,
    pub notifications: bool,
    pub updated_count: u64,
    pub last_action_slot: u64,
}
