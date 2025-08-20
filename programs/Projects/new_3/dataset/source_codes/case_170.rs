use anchor_lang::prelude::*;
declare_id!("IoTAccess1111111111111111111111111111111");

/// IoTデバイス管理情報
#[account]
pub struct Device {
    pub owner:      Pubkey,  // デバイス所有者
    pub label:      String,  // デバイス名
    pub log_count:  u64,     // ログエントリ数
}

/// アクセスログ記録
#[account]
pub struct AccessLog {
    pub user:       Pubkey,  // アクセスしたユーザー
    pub device:     Pubkey,  // 本来は Device.key() と一致すべき
    pub message:    String,  // ログメッセージ
}

#[derive(Accounts)]
pub struct RegisterDevice<'info> {
    #[account(init, payer = owner, space = 8 + 32 + 4 + 64 + 8)]
    pub device:     Account<'info, Device>,
    #[account(mut)]
    pub owner:      Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct LogAccess<'info> {
    /// Device.owner == owner.key() は検証される
    #[account(mut, has_one = owner)]
    pub device:     Account<'info, Device>,

    /// AccessLog.device ⇔ device.key() の検証がない
    #[account(init, payer = user, space = 8 + 32 + 32 + 4 + 128)]
    pub log:        Account<'info, AccessLog>,

    #[account(mut)]
    pub user:       Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct AppendLog<'info> {
    /// AccessLog.user == user.key() は検証される
    #[account(mut, has_one = user)]
    pub log:        Account<'info, AccessLog>,

    /// device.key() と log.device の整合性チェックがない
    #[account(mut)]
    pub device:     Account<'info, Device>,

    pub user:       Signer<'info>,
}

#[program]
pub mod iot_logging_vuln {
    use super::*;

    /// デバイスを登録
    pub fn register_device(ctx: Context<RegisterDevice>, label: String) -> Result<()> {
        let d = &mut ctx.accounts.device;
        d.owner     = ctx.accounts.owner.key();
        d.label     = label;
        d.log_count = 0;
        Ok(())
    }

    /// アクセスログを作成
    pub fn log_access(ctx: Context<LogAccess>, initial_msg: String) -> Result<()> {
        let d = &mut ctx.accounts.device;
        let l = &mut ctx.accounts.log;

        // 脆弱性ポイント:
        // l.device = d.key(); と設定しているのみで、
        // AccessLog.device と Device.key() の検証がない
        l.user    = ctx.accounts.user.key();
        l.device  = d.key();
        l.message = initial_msg;
        // ログ数を増やす
        d.log_count = d.log_count.checked_add(1).unwrap_or(d.log_count);
        Ok(())
    }

    /// 既存ログに追記
    pub fn append_log(ctx: Context<AppendLog>, extra: String) -> Result<()> {
        let d = &mut ctx.accounts.device;
        let l = &mut ctx.accounts.log;

        // 本来必要：
        // require_keys_eq!(l.device, d.key(), ErrorCode::DeviceMismatch);

        // ログメッセージの末尾に追記
        let mut buf = String::new();
        buf.push_str(&l.message);
        buf.push_str(" | ");
        buf.push_str(&extra);
        l.message = buf;

        // デバイス側のカウンタも更新
        d.log_count = d.log_count.checked_add(1).unwrap_or(d.log_count);
        Ok(())
    }
}

#[error_code]
pub enum ErrorCode {
    #[msg("AccessLog が指定の Device と一致しません")]
    DeviceMismatch,
}
