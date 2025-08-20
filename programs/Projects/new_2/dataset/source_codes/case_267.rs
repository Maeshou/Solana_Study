use anchor_lang::prelude::*;

#[program]
pub mod profile_manager {
    use super::*;
    pub fn update_profile(
        ctx: Context<UpdateProfile>,
        new_email: String,
        new_name: String,
    ) -> Result<()> {
        let profile = &mut ctx.accounts.profile_data;

        // 更新前の履歴をためる
        profile.history.push((profile.email.clone(), profile.username.clone()));
        profile.email = new_email;
        profile.username = new_name;

        // 変更の合図だけ出力
        msg!("Profile data was modified by {}", ctx.accounts.user.key());
        Ok(())
    }
}

#[derive(Accounts)]
pub struct UpdateProfile<'info> {
    #[account(mut)]
    pub profile_data: Account<'info, ProfileData>,
    pub user: Signer<'info>,
    #[account(mut)]
    /// CHECK: ログ記録用（制約なし）
    pub audit_log: AccountInfo<'info>,
}

#[account]
pub struct ProfileData {
    pub email: String,
    pub username: String,
    pub history: Vec<(String, String)>,
}
