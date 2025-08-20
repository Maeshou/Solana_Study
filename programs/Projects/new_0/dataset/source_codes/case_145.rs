use anchor_lang::prelude::*;

/// ユーザープロファイル設定管理プログラム
declare_id!("ProfS111111111111111111111111111111111");

#[program]
pub mod profile_settings {
    /// プロファイル設定アカウントの作成
    pub fn init_profile(ctx: Context<InitProfile>, username: String) -> Result<()> {
        let profile = &mut ctx.accounts.profile;
        // ユーザー名長チェック
        require!(username.len() <= 32, ErrorCode::UsernameTooLong);
        profile.owner = ctx.accounts.user.key();
        profile.username = username;
        profile.dark_mode = false; // デフォルトはオフ
        Ok(())
    }

    /// プロフィールを更新
    pub fn update_profile(ctx: Context<UpdateProfile>, new_username: String, dark_mode: bool) -> Result<()> {
        let profile = &mut ctx.accounts.profile;
        // 所有者チェック
        require!(profile.owner == ctx.accounts.user.key(), ErrorCode::AccessDenied);
        // ユーザー名長チェック
        require!(new_username.len() <= 32, ErrorCode::UsernameTooLong);
        
        profile.username = new_username;
        profile.dark_mode = dark_mode;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitProfile<'info> {
    /// プロファイルアカウントを初期化
    #[account(init, payer = user, space = 8 + 32 + 4 + 32 + 1)]
    pub profile: Account<'info, Profile>,
    #[account(mut)]
    pub user:    Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateProfile<'info> {
    /// 既存プロファイルの参照
    #[account(mut)]
    pub profile: Account<'info, Profile>,
    pub user:    Signer<'info>,
}

#[account]
pub struct Profile {
    /// プロファイル所有者
    pub owner:     Pubkey,
    /// ユーザー名（最大32文字）
    pub username:  String,
    /// ダークモード設定
    pub dark_mode: bool,
}

#[error_code]
pub enum ErrorCode {
    #[msg("アクセス権限がありません")] AccessDenied,
    #[msg("ユーザー名が長すぎます")] UsernameTooLong,
}
