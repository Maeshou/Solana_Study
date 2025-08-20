use anchor_lang::prelude::*;
use anchor_lang::solana_program::sysvar::clock::Clock;

// APIキー管理プログラム
declare_id!("ApiK111111111111111111111111111111111111");

#[program]
pub mod api_key_manager {
    /// 新規 API キー発行
    pub fn init_key(
        ctx: Context<InitKey>,
        key: [u8; 32],
    ) -> Result<()> {
        // 空のキーは禁止
        require!(key != [0u8; 32], ErrorCode::InvalidKey);

        let acct = &mut ctx.accounts.key_account;
        acct.owner        = ctx.accounts.user.key();
        acct.key          = key;
        acct.active       = true;
        acct.last_rotated = 0;
        Ok(())
    }

    /// API キーのローテーション
    pub fn rotate_key(
        ctx: Context<ModifyKey>,
        new_key: [u8; 32],
    ) -> Result<()> {
        // 空のキーは禁止
        require!(new_key != [0u8; 32], ErrorCode::InvalidKey);

        let acct = &mut ctx.accounts.key_account;
        // 所有者のみ操作可能
        require!(acct.owner == ctx.accounts.user.key(), ErrorCode::AccessDenied);

        acct.key = new_key;
        // 現在時刻を last_rotated に設定
        acct.last_rotated = Clock::get()?.unix_timestamp;
        Ok(())
    }

    /// API キーの無効化
    pub fn revoke_key(
        ctx: Context<ModifyKey>
    ) -> Result<()> {
        let acct = &mut ctx.accounts.key_account;
        // 所有者のみ操作可能
        require!(acct.owner == ctx.accounts.user.key(), ErrorCode::AccessDenied);

        acct.active = false;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitKey<'info> {
    /// 再初期化を防止
    #[account(init, payer = user, space = 8 + 32 + 32 + 1 + 8)]
    pub key_account: Account<'info, KeyAccount>,
    #[account(mut)]
    pub user:        Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ModifyKey<'info> {
    /// Account<> 型で所有者チェック
    #[account(mut)]
    pub key_account: Account<'info, KeyAccount>,
    #[account(mut)]
    pub user:        Signer<'info>,
    pub clock:       Sysvar<'info, Clock>,
}

#[account]
pub struct KeyAccount {
    /// 操作可能なユーザー
    pub owner:        Pubkey,
    /// 実際の API キー
    pub key:          [u8; 32],
    /// キーが有効か
    pub active:       bool,
    /// 最終ローテーション時刻 (UNIX)
    pub last_rotated: i64,
}

#[error_code]
pub enum ErrorCode {
    #[msg("アクセス拒否")] AccessDenied,
    #[msg("無効なキーです")] InvalidKey,
}
