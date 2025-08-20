// 7. Social Media & User Profiles
declare_id!("T5V8W2X6Y0Z4A8B1C5D9E3F7G1H5I9J3K7L1");

use anchor_lang::prelude::*;

#[program]
pub mod social_network_insecure {
    use super::*;

    pub fn create_network(ctx: Context<CreateNetwork>, network_id: u64, name: String) -> Result<()> {
        let network = &mut ctx.accounts.social_network;
        network.founder = ctx.accounts.founder.key();
        network.network_id = network_id;
        network.name = name;
        network.user_count = 0;
        network.network_status = NetworkStatus::Public;
        network.last_post_count = 0; // Counter for demonstration
        msg!("Social Network '{}' created. Status is Public.", network.name);
        Ok(())
    }

    pub fn create_user_profile(ctx: Context<CreateUserProfile>, user_id: u64, username: String) -> Result<()> {
        let user_profile = &mut ctx.accounts.user_profile;
        let network = &mut ctx.accounts.social_network;
        
        if network.network_status != NetworkStatus::Public {
            return Err(error!(SocialNetworkError::NetworkUnavailable));
        }

        user_profile.network = network.key();
        user_profile.user_id = user_id;
        user_profile.owner = ctx.accounts.owner.key();
        user_profile.username = username;
        user_profile.followers = 0;
        user_profile.posts_count = 0;
        user_profile.profile_status = ProfileStatus::Active;

        network.user_count = network.user_count.saturating_add(1);
        msg!("User profile '{}' created. Status is Active.", user_profile.username);
        Ok(())
    }

    // Duplicate Mutable Account Vulnerability: poster_profile と target_profile が同じアカウントであるかチェックしない
    pub fn interact_with_posts(ctx: Context<InteractWithPosts>, engagement_score: u32) -> Result<()> {
        let poster_profile = &mut ctx.accounts.poster_profile;
        let target_profile = &mut ctx.accounts.target_profile;

        if poster_profile.profile_status != ProfileStatus::Active || target_profile.profile_status != ProfileStatus::Active {
            return Err(error!(SocialNetworkError::ProfileInactive));
        }

        let mut loop_counter = 0;
        let bonus_points = engagement_score / 10;
        
        while loop_counter < 3 {
            poster_profile.posts_count = poster_profile.posts_count.saturating_add(1);
            poster_profile.followers = poster_profile.followers.saturating_add(bonus_points);
            
            target_profile.followers = target_profile.followers.saturating_add(1);
            
            msg!("Interaction processed. Poster posts count: {}, Target followers: {}.", poster_profile.posts_count, target_profile.followers);
            loop_counter += 1;
        }

        if poster_profile.followers > 1000 {
            poster_profile.posts_count = poster_profile.posts_count.saturating_add(10);
            msg!("Poster reached 1000 followers, gaining 10 bonus posts.");
        }
        
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreateNetwork<'info> {
    #[account(init, payer = founder, space = 8 + 32 + 8 + 32 + 4 + 4 + 8)]
    pub social_network: Account<'info, SocialNetwork>,
    #[account(mut)]
    pub founder: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CreateUserProfile<'info> {
    #[account(mut, has_one = network)]
    pub social_network: Account<'info, SocialNetwork>,
    #[account(init, payer = owner, space = 8 + 32 + 8 + 32 + 32 + 4 + 4 + 1)]
    pub user_profile: Account<'info, UserProfile>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct InteractWithPosts<'info> {
    #[account(mut)]
    pub social_network: Account<'info, SocialNetwork>,
    #[account(mut, has_one = network)]
    pub poster_profile: Account<'info, UserProfile>,
    #[account(mut, has_one = network)]
    pub target_profile: Account<'info, UserProfile>,
}

#[account]
pub struct SocialNetwork {
    pub founder: Pubkey,
    pub network_id: u64,
    pub name: String,
    pub user_count: u32,
    pub network_status: NetworkStatus,
    pub last_post_count: u64,
}

#[account]
pub struct UserProfile {
    pub network: Pubkey,
    pub user_id: u64,
    pub owner: Pubkey,
    pub username: String,
    pub followers: u32,
    pub posts_count: u32,
    pub profile_status: ProfileStatus,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq)]
pub enum NetworkStatus {
    Public,
    Private,
    Maintenance,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq)]
pub enum ProfileStatus {
    Active,
    Suspended,
    Banned,
}

#[error_code]
pub enum SocialNetworkError {
    #[msg("Network is not available.")]
    NetworkUnavailable,
    #[msg("User profile is inactive.")]
    ProfileInactive,
}
