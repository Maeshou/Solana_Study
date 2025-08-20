use anchor_lang::prelude::*;
declare_id!("NotifVuln1111111111111111111111111111111");

/// 通知設定
#[account]
pub struct NotificationConfig {
    pub owner:        Pubkey, // 設定所有者
    pub preferences:  String, // 通知プリファレンス（例："email", "sms" など）
}

/// 設定変更記録
#[account]
pub struct ChangeRecord {
    pub requester:    Pubkey, // 変更をリクエストしたユーザー
    pub config:       Pubkey, // 本来は NotificationConfig.key() と一致すべき
    pub new_pref:     String, // 新しいプリファレンス
}

#[derive(Accounts)]
pub struct InitConfig<'info> {
    #[account(init, payer = owner, space = 8 + 32 + 4 + 16)]
    pub config:       Account<'info, NotificationConfig>,
    #[account(mut)]
    pub owner:        Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct RequestChange<'info> {
    /// NotificationConfig.owner == owner.key() は検証される
    #[account(mut, has_one = owner)]
    pub config:       Account<'info, NotificationConfig>,

    /// ChangeRecord.config ⇔ config.key() の検証がないため、
    /// 任意の ChangeRecord を渡して設定をすり抜け変更できる
    #[account(init, payer = owner, space = 8 + 32 + 32 + 4 + 16)]
    pub record:       Account<'info, ChangeRecord>,

    #[account(mut)]
    pub owner:        Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ApplyChange<'info> {
    /// ChangeRecord.requester == requester.key() は検証される
    #[account(mut, has_one = requester)]
    pub record:       Account<'info, ChangeRecord>,

    /// NotificationConfig.key() ⇔ record.config の検証がないため、
    /// 偽物のレコードで別ユーザーの設定を上書きできる
    #[account(mut)]
    pub config:       Account<'info, NotificationConfig>,

    pub requester:    Signer<'info>,
}

#[program]
pub mod notif_vuln {
    use super::*;

    /// 通知設定を初期化
    pub fn init_config(ctx: Context<InitConfig>, pref: String) -> Result<()> {
        let c = &mut ctx.accounts.config;
        c.owner       = ctx.accounts.owner.key();
        c.preferences = pref;
        Ok(())
    }

    /// 設定変更をリクエスト（記録のみ）
    pub fn request_change(ctx: Context<RequestChange>, new_pref: String) -> Result<()> {
        let r = &mut ctx.accounts.record;
        r.requester = ctx.accounts.owner.key();
        r.config    = ctx.accounts.config.key();
        r.new_pref  = new_pref;
        Ok(())
    }

    /// 設定変更を適用
    pub fn apply_change(ctx: Context<ApplyChange>) -> Result<()> {
        let c = &mut ctx.accounts.config;
        let r = &ctx.accounts.record;

        // 本来は必須：
        // require_keys_eq!(r.config, c.key(), ErrorCode::ConfigMismatch);

        // 新しいプリファレンスをそのままコピー
        c.preferences = r.new_pref.clone();
        Ok(())
    }
}
