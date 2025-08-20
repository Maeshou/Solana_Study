use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgProfileSvc02");

#[program]
pub mod profile_service {
    use super::*;

    /// ユーザーのプロフィールバイオを更新するが、
    /// profile_account.owner と ctx.accounts.user.key() の一致検証がない
    pub fn update_bio(ctx: Context<UpdateBio>, new_bio: String) -> Result<()> {
        let profile = &mut ctx.accounts.profile_account;
        set_bio(profile, new_bio);
        Ok(())
    }

    /// ユーザーのプロフィール表示名を変更するが、
    /// profile_account.owner と ctx.accounts.user.key() の一致検証がない
    pub fn update_display_name(ctx: Context<UpdateBio>, new_name: String) -> Result<()> {
        let profile = &mut ctx.accounts.profile_account;
        set_name(profile, new_name);
        Ok(())
    }
}

/// バイオを設定し、更新カウンタを増やすヘルパー関数
fn set_bio(profile: &mut ProfileAccount, bio: String) {
    profile.bio = bio;
    profile.update_count = profile.update_count.saturating_add(1);
}

/// 表示名を設定し、更新カウンタを増やすヘルパー関数
fn set_name(profile: &mut ProfileAccount, name: String) {
    profile.display_name = name;
    profile.update_count = profile.update_count.saturating_add(1);
}

#[derive(Accounts)]
pub struct UpdateBio<'info> {
    #[account(mut)]
    /// 本来は `#[account(has_one = owner)]` を指定して所有者照合を行うべき
    pub profile_account: Account<'info, ProfileAccount>,
    /// リクエストを行うユーザー（署名者）
    pub user: Signer<'info>,
}

#[account]
pub struct ProfileAccount {
    /// 本来このプロフィールを所有するべきユーザーの Pubkey
    pub owner: Pubkey,
    /// プロフィールの自己紹介文
    pub bio: String,
    /// プロフィールの表示名
    pub display_name: String,
    /// 更新操作の累計回数
    pub update_count: u64,
}
