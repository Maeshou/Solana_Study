use anchor_lang::prelude::*;

declare_id!("UserPrf1111111111111111111111111111111111");

#[program]
pub mod user_profile {
    /// プロファイル作成
    pub fn create_profile(ctx: Context<CreateProfile>, display_name: String) -> Result<()> {
        // 表示名長チェック（オーバーフロー防止）
        require!(display_name.len() <= 32, ErrorCode::NameTooLong);

        let profile = &mut ctx.accounts.profile;
        profile.owner        = ctx.accounts.user.key();  // Signer Authorization
        profile.display_name = display_name;
        Ok(())
    }

    /// プロファイル更新
    pub fn update_profile(ctx: Context<UpdateProfile>, new_name: String) -> Result<()> {
        // 表示名長チェック
        require!(new_name.len() <= 32, ErrorCode::NameTooLong);

        let profile = &mut ctx.accounts.profile;
        // Account Matching / Signer Authorization
        require!(
            profile.owner == ctx.accounts.user.key(),
            ErrorCode::Unauthorized
        );
        profile.display_name = new_name;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreateProfile<'info> {
    /// 新規作成：init 制約で再初期化（Reinit Attack）を防止
    #[account(init, payer = user, space = 8 + 32 + 32)]
    pub profile: Account<'info, Profile>,

    /// このトランザクションを署名しているユーザー
    #[account(mut)]
    pub user:    Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateProfile<'info> {
    /// 既存の Profile アカウント（Account<> による Owner Check／Type Cosplay）
    #[account(mut)]
    pub profile: Account<'info, Profile>,

    /// 実際に署名したユーザー（Signer Authorization + Account Matching）
    pub user:    Signer<'info>,
}

#[account]
pub struct Profile {
    /// このプロファイルを操作できるユーザー
    pub owner:        Pubkey,
    /// 表示名（最大32文字）
    pub display_name: String,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Display name is too long")]
    NameTooLong,
    #[msg("Unauthorized")]
    Unauthorized,
}
