use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod user_profile_manager {
    use super::*;
    pub fn create_profile(ctx: Context<CreateProfile>, name: String, bio: String) -> Result<()> {
        let profile = &mut ctx.accounts.user_profile;
        profile.authority = *ctx.accounts.user.key;
        profile.name = name;
        profile.bio = bio;
        Ok(())
    }

    pub fn update_bio(ctx: Context<UpdateProfile>, new_bio: String) -> Result<()> {
        let profile = &mut ctx.accounts.user_profile;
        profile.bio = new_bio;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreateProfile<'info> {
    #[account(
        init,
        payer = user,
        space = 8 + 32 + 4 + 50 + 4 + 280, // Discriminator + authority + name_len + name + bio_len + bio
        seeds = [b"profile", user.key().as_ref()],
        bump
    )]
    pub user_profile: Account<'info, UserProfile>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateProfile<'info> {
    // このアカウントが現在のプログラムによって所有されていることを自動で検証
    #[account(
        mut,
        seeds = [b"profile", user.key().as_ref()],
        bump,
        has_one = user // 権限者(authority)のチェックも追加
    )]
    pub user_profile: Account<'info, UserProfile>,
    pub user: Signer<'info>,
}

#[account]
pub struct UserProfile {
    pub authority: Pubkey,
    pub name: String,
    pub bio: String,
}