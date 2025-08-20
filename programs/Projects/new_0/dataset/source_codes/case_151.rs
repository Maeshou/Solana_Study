use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWxTWq4rove6YjFgDhqyQ5RBwzT5");

#[program]
pub mod profile_manager {
    use super::*;

    /// プロフィール初期化：username を受け取り、PDA を生成
    pub fn initialize_profile(
        ctx: Context<InitializeProfile>,
        bump: u8,
        username: String,
    ) -> Result<()> {
        // ユーザー名の最大長チェック
        if username.chars().count() > 32 {
            return Err(ErrorCode::UsernameTooLong.into());
        }
        let profile = &mut ctx.accounts.profile;
        profile.owner = ctx.accounts.user.key();
        profile.bump = bump;
        profile.username = username;
        Ok(())
    }

    /// ユーザー名更新：オーナー署名＆has_one で検証
    pub fn update_username(
        ctx: Context<UpdateUsername>,
        new_username: String,
    ) -> Result<()> {
        // 長すぎる更新を防止
        if new_username.chars().count() > 32 {
            return Err(ErrorCode::UsernameTooLong.into());
        }
        let profile = &mut ctx.accounts.profile;
        profile.username = new_username;
        Ok(())
    }
}

/// アカウント定義：プロフィール初期化用
#[derive(Accounts)]
#[instruction(bump: u8)]
pub struct InitializeProfile<'info> {
    /// PDA で生成する Profile
    #[account(
        init,
        payer = user,
        space = 8 + 32 + 1 + 4 + 32,  // discriminator + owner + bump + string len prefix + max 32 chars
        seeds = [b"profile", user.key().as_ref()],
        bump
    )]
    pub profile: Account<'info, Profile>,

    /// トランザクション送信者（オーナー）
    #[account(mut)]
    pub user: Signer<'info>,

    pub system_program: Program<'info, System>,
}

/// アカウント定義：ユーザー名更新用
#[derive(Accounts)]
pub struct UpdateUsername<'info> {
    /// 既存の Profile（オーナー & PDA 検証 + has_one）
    #[account(
        mut,
        seeds = [b"profile", profile.owner.as_ref()],
        bump = profile.bump,
        has_one = owner
    )]
    pub profile: Account<'info, Profile>,

    /// Profile 所有者（署名必須）
    #[account(signer)]
    pub owner: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
}

/// Profile データ構造：オーナー、bump、username を保持
#[account]
pub struct Profile {
    pub owner: Pubkey,
    pub bump: u8,
    pub username: String,
}

/// カスタムエラー定義
#[error_code]
pub enum ErrorCode {
    #[msg("Username exceeds maximum length of 32 characters")]
    UsernameTooLong,
}
