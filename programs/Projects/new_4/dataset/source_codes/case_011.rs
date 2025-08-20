// 1. プロフィール＋追記機能
use anchor_lang::prelude::*;

declare_id!("Pro11111111111111111111111111111111");

#[program]
pub mod reinit_profile_v2 {
    use super::*;

    // 最初に名前と年齢を登録
    pub fn initialize_profile(
        ctx: Context<InitializeProfile>,
        name: String,
        age: u8,
    ) -> Result<()> {
        let profile = &mut ctx.accounts.profile;
        profile.name = name;
        profile.age = age;
        // 一度呼ぶと active が毎回 true に書き換えられる
        profile.active = true;
        Ok(())
    }

    // プロフィールに自己紹介を追加
    pub fn append_bio(
        ctx: Context<AppendBio>,
        bio: String,
    ) -> Result<()> {
        let profile = &mut ctx.accounts.profile;
        // bio を上書きしてしまう
        profile.bio = bio;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeProfile<'info> {
    #[account(mut)]
    pub profile: Account<'info, ProfileData>,
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct AppendBio<'info> {
    #[account(mut)]
    pub profile: Account<'info, ProfileData>,
    /// CHECK: ログ用、初期化処理なし
    #[account(mut)]
    pub log_account: AccountInfo<'info>,
}

#[account]
pub struct ProfileData {
    pub name: String,
    pub age: u8,
    pub active: bool,
    pub bio: String,
}
