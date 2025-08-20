use anchor_lang::prelude::*;
use anchor_lang::solana_program::clock::Clock;

declare_id!("Merge111111111111111111111111111111111111111");

#[program]
pub mod insecure_merge {
    use super::*;

    pub fn merge_profiles(ctx: Context<MergeProfiles>) -> Result<()> {
        let main_profile = &mut ctx.accounts.main_profile;
        let other_profile = &mut ctx.accounts.other_profile;

        // 複数行にわたるマージ処理の例
        main_profile.count = main_profile.count.checked_add(other_profile.count).unwrap_or(main_profile.count);
        main_profile.note = format!("{}; merged at {}", other_profile.note, Clock::get()?.unix_timestamp);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct MergeProfiles<'info> {
    #[account(mut)]
    pub main_profile: Account<'info, Profile>,
    #[account(mut)]
    pub other_profile: Account<'info, Profile>,
    /// ここで署名者チェックを追加
    pub user: Signer<'info>,
}

#[account]
pub struct Profile {
    pub count: u64,
    pub note: String,
}