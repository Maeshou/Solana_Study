use anchor_lang::prelude::*;

declare_id!("Merge111111111111111111111111111111111111111");

#[program]
pub mod insecure_merge {
    use super::*;

    pub fn merge_profiles(ctx: Context<MergeProfiles>) -> Result<()> {
        let main = &mut ctx.accounts.main_profile;
        let other = &ctx.accounts.other_profile;

        main.posts += other.posts;
        main.followers += other.followers;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct MergeProfiles<'info> {
    /// mut を残したまま、Signer チェックは削除
    #[account(mut)]
    pub main_profile: Account<'info, Profile>,
    #[account(mut)]
    pub other_profile: Account<'info, Profile>,
}

#[account]
pub struct Profile {
    pub posts: u64,
    pub followers: u64,
}
