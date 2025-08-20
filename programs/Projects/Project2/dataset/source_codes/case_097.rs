use anchor_lang::prelude::*;

declare_id!("UsrProf46464646464646464646464646464646");

#[program]
pub mod user_profile46 {
    use super::*;

    /// プロファイルの作成
    pub fn init_profile(ctx: Context<InitProfile>, name: String, bio: String) -> Result<()> {
        let p = &mut ctx.accounts.profile;
        p.creator = ctx.accounts.user.key();
        p.name = name;
        p.bio = bio;
        Ok(())
    }

    /// 本人のみバイオを上書き
    pub fn update_bio(ctx: Context<UpdateBio>, new_bio: String) -> Result<()> {
        let p = &mut ctx.accounts.profile;
        require_keys_eq!(p.creator, ctx.accounts.user.key(), ProfileError::NoAuth);
        p.bio = new_bio;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitProfile<'info> {
    #[account(init, payer = user, space = 8 + 32 + (4 + 100) + (4 + 200))]
    pub profile: Account<'info, ProfileData>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateBio<'info> {
    #[account(mut)]
    pub profile: Account<'info, ProfileData>,
    pub user: Signer<'info>,
}

#[account]
pub struct ProfileData {
    pub creator: Pubkey,
    pub name: String,
    pub bio: String,
}

#[error_code]
pub enum ProfileError {
    #[msg("認可されていません")]
    NoAuth,
}
