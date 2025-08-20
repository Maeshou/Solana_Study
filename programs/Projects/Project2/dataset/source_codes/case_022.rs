// =====================================
// 2. User Profile Program (AccountInfo使用)
// =====================================
use anchor_lang::prelude::*;

declare_id!("22222222222222222222222222222222");

#[program]
pub mod secure_user_profile {
    use super::*;

    pub fn create_profile(
        ctx: Context<CreateProfile>,
        name: String,
        bio: String,
    ) -> Result<()> {
        // AccountInfoを使用しつつ、安全なowner checkを実装
        let profile_account_info = ctx.accounts.profile.to_account_info();
        
        // プログラムが所有者であることを確認
        require!(
            profile_account_info.owner == ctx.program_id,
            ErrorCode::InvalidProfileOwner
        );

        let profile = &mut ctx.accounts.profile;
        profile.authority = ctx.accounts.authority.key();
        profile.name = name;
        profile.bio = bio;
        profile.created_at = Clock::get()?.unix_timestamp;

        Ok(())
    }

    pub fn update_profile(
        ctx: Context<UpdateProfile>,
        name: Option<String>,
        bio: Option<String>,
    ) -> Result<()> {
        // AccountInfoでowner checkを実装
        let profile_info = ctx.accounts.profile.to_account_info();
        require!(
            profile_info.owner == ctx.program_id,
            ErrorCode::InvalidProfileOwner
        );

        let profile = &mut ctx.accounts.profile;
        
        if let Some(new_name) = name {
            profile.name = new_name;
        }
        if let Some(new_bio) = bio {
            profile.bio = new_bio;
        }

        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(name: String, bio: String)]
pub struct CreateProfile<'info> {
    #[account(
        init,
        payer = authority,
        space = 8 + 32 + 4 + name.len() + 4 + bio.len() + 8,
        constraint = profile.to_account_info().owner == program_id
    )]
    pub profile: Account<'info, UserProfile>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateProfile<'info> {
    #[account(
        mut,
        has_one = authority,
        constraint = profile.to_account_info().owner == program_id
    )]
    pub profile: Account<'info, UserProfile>,
    pub authority: Signer<'info>,
}

#[account]
pub struct UserProfile {
    pub authority: Pubkey,
    pub name: String,
    pub bio: String,
    pub created_at: i64,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Invalid profile account owner")]
    InvalidProfileOwner,
}
