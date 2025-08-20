use anchor_lang::prelude::*;

// Program ID - replace with your own
declare_id!("Fg6PaFpoGXkYsidMpW1e2x3y4z5A6B7C8D9E0F1G2H3I");

#[program]
pub mod safe_profile_manager {
    use super::*;

    /// Initialize a user profile PDA with an empty bio
    pub fn initialize_profile(
        ctx: Context<InitializeProfile>,
        bump: u8,
    ) -> ProgramResult {
        let profile = &mut ctx.accounts.profile;
        profile.owner = *ctx.accounts.user.key;
        profile.bump = bump;
        profile.bio = String::new();
        Ok(())
    }

    /// Update the bio for the profile, enforcing a maximum length
    pub fn update_bio(
        ctx: Context<UpdateBio>,
        bump: u8,
        new_bio: String,
    ) -> ProgramResult {
        require!(new_bio.len() <= 280, ErrorCode::BioTooLong);
        let profile = &mut ctx.accounts.profile;
        profile.bio = new_bio;
        Ok(())
    }

    /// Delete the profile account, returning lamports to the owner
    pub fn delete_profile(
        ctx: Context<DeleteProfile>,
        bump: u8,
    ) -> ProgramResult {
        // Using `close` on account will transfer lamports back to the owner
        Ok(())
    }
}

// Context definitions
#[derive(Accounts)]
#[instruction(bump: u8)]
pub struct InitializeProfile<'info> {
    #[account(
        init,
        payer = user,
        seeds = [b"profile", user.key().as_ref()],
        bump = bump,
        space = 8 + 32 + 1 + 4 + 280, // discriminator + owner + bump + String length prefix + max bio
    )]
    pub profile: Account<'info, Profile>,

    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(bump: u8)]
pub struct UpdateBio<'info> {
    #[account(
        mut,
        seeds = [b"profile", owner.key().as_ref()],
        bump = bump,
        has_one = owner @ ErrorCode::Unauthorized,
        constraint = profile.owner == *owner.key @ ErrorCode::Unauthorized,
    )]
    pub profile: Account<'info, Profile>,
    pub owner: Signer<'info>,
}

#[derive(Accounts)]
#[instruction(bump: u8)]
pub struct DeleteProfile<'info> {
    #[account(
        mut,
        close = owner,
        seeds = [b"profile", owner.key().as_ref()],
        bump = bump,
        has_one = owner @ ErrorCode::Unauthorized,
    )]
    pub profile: Account<'info, Profile>,
    pub owner: Signer<'info>,
}

// Account data
#[account]
pub struct Profile {
    pub owner: Pubkey,
    pub bump: u8,
    pub bio: String,
}

// Custom error codes
#[error]
pub enum ErrorCode {
    #[msg("You are not authorized to perform this action.")]
    Unauthorized,
    #[msg("Bio exceeds maximum length of 280 characters.")]
    BioTooLong,
}
