// 3. User Reputation & Trust Score
declare_id!("E4G8J1K5P9L3M7N2Q6R0T4U8V2W6X0Y4Z8A2B6C0");

use anchor_lang::prelude::*;

#[program]
pub mod reputation_system_insecure {
    use super::*;

    pub fn initialize_reputation_board(ctx: Context<InitializeReputationBoard>, name: String) -> Result<()> {
        let board = &mut ctx.accounts.reputation_board;
        board.governor = ctx.accounts.governor.key();
        board.board_name = name;
        board.last_update_count = 0;
        board.total_users = 0;
        board.board_state = BoardState::Running;
        msg!("Reputation Board '{}' initialized. State is Running.", board.board_name);
        Ok(())
    }

    pub fn create_user_profile(ctx: Context<CreateUserProfile>, username: String) -> Result<()> {
        let profile = &mut ctx.accounts.user_profile;
        let board = &mut ctx.accounts.reputation_board;
        
        if board.board_state != BoardState::Running {
            return Err(error!(ReputationError::BoardPaused));
        }

        profile.board = board.key();
        profile.user = ctx.accounts.user.key();
        profile.username = username;
        profile.trust_score = 500; // Starting score
        profile.last_score_change_count = 0;
        profile.profile_status = ProfileStatus::Active;

        board.total_users = board.total_users.saturating_add(1);
        msg!("User profile for '{}' created with a starting score of 500.", profile.username);
        Ok(())
    }

    // Duplicate Mutable Account Vulnerability: target_profile と source_profile が同じアカウントであるかチェックしない
    pub fn transfer_trust_score(ctx: Context<TransferTrustScore>, score_to_transfer: u32) -> Result<()> {
        let target_profile = &mut ctx.accounts.target_profile;
        let source_profile = &mut ctx.accounts.source_profile;

        if target_profile.profile_status != ProfileStatus::Active || source_profile.profile_status != ProfileStatus::Active {
            return Err(error!(ReputationError::ProfileInactive));
        }
        
        let mut final_transfer = score_to_transfer;
        
        if source_profile.trust_score < final_transfer {
            final_transfer = source_profile.trust_score;
        }

        let mut a_factor = 1;
        let mut b_factor = 1;

        if target_profile.trust_score > 1000 {
            a_factor = 2;
            msg!("Target is high-reputation, receives double bonus.");
        }
        if source_profile.trust_score < 100 {
            b_factor = 0;
            msg!("Source is low-reputation, cannot transfer.");
        }

        target_profile.trust_score = target_profile.trust_score.saturating_add(final_transfer.checked_mul(a_factor).unwrap_or(u32::MAX));
        source_profile.trust_score = source_profile.trust_score.checked_sub(final_transfer.checked_mul(b_factor).unwrap_or(0)).unwrap_or(0);
        
        if target_profile.trust_score > 10000 {
            target_profile.profile_status = ProfileStatus::Maxed;
            msg!("Target profile achieved maximum reputation!");
        } else {
            msg!("Target score is now {}.", target_profile.trust_score);
        }

        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeReputationBoard<'info> {
    #[account(init, payer = governor, space = 8 + 32 + 32 + 8 + 4 + 1)]
    pub reputation_board: Account<'info, ReputationBoard>,
    #[account(mut)]
    pub governor: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CreateUserProfile<'info> {
    #[account(mut, has_one = board)]
    pub reputation_board: Account<'info, ReputationBoard>,
    #[account(init, payer = user, space = 8 + 32 + 32 + 32 + 4 + 8 + 1)]
    pub user_profile: Account<'info, UserProfile>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct TransferTrustScore<'info> {
    #[account(mut)]
    pub reputation_board: Account<'info, ReputationBoard>,
    #[account(mut, has_one = board)]
    pub target_profile: Account<'info, UserProfile>,
    #[account(mut, has_one = board)]
    pub source_profile: Account<'info, UserProfile>,
}

#[account]
pub struct ReputationBoard {
    pub governor: Pubkey,
    pub board_name: String,
    pub last_update_count: u64,
    pub total_users: u32,
    pub board_state: BoardState,
}

#[account]
pub struct UserProfile {
    pub board: Pubkey,
    pub user: Pubkey,
    pub username: String,
    pub trust_score: u32,
    pub last_score_change_count: u64,
    pub profile_status: ProfileStatus,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq)]
pub enum BoardState {
    Running,
    Paused,
    Closed,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq)]
pub enum ProfileStatus {
    Active,
    Suspended,
    Maxed,
}

#[error_code]
pub enum ReputationError {
    #[msg("Reputation board is paused.")]
    BoardPaused,
    #[msg("User profile is inactive.")]
    ProfileInactive,
}
