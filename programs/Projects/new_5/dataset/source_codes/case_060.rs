// 3. User Reputation & Trust Score
declare_id!("E4G8J1K5P9L3M7N2Q6R0T4U8V2W6X0Y4Z8A2B6C0");

use anchor_lang::prelude::*;

#[program]
pub mod reputation_system_insecure {
    use super::*;

    // ループも分岐も使わないパターン
    pub fn initialize_reputation_board(ctx: Context<InitializeReputationBoard>, board_name: String, initial_user_count: u32) -> Result<()> {
        let board = &mut ctx.accounts.reputation_board;
        board.governor = ctx.accounts.governor.key();
        board.board_name = board_name;
        board.last_update_count = 0;
        board.total_users = initial_user_count.checked_add(50).unwrap_or(u32::MAX);
        board.board_state = BoardState::Running;
        msg!("Reputation Board '{}' initialized with {} users. State is Running.", board.board_name, board.total_users);
        Ok(())
    }

    pub fn create_user_profile(ctx: Context<CreateUserProfile>, username: String, trust_score: u32) -> Result<()> {
        let profile = &mut ctx.accounts.user_profile;
        let board = &mut ctx.accounts.reputation_board;
        
        // 分岐の具体例: ボードの状態に応じて初期スコアを調整
        if board.board_state == BoardState::Running {
            profile.trust_score = trust_score;
        } else {
            profile.trust_score = 0;
            msg!("Board is not running. User profile created with trust score 0.");
        }

        profile.board = board.key();
        profile.user = ctx.accounts.user.key();
        profile.username = username;
        profile.last_score_change_count = trust_score as u64;
        profile.profile_status = ProfileStatus::Active;

        board.total_users = board.total_users.saturating_add(1);
        msg!("User profile for '{}' created with a starting score of {}.", profile.username, profile.trust_score);
        Ok(())
    }

    // Duplicate Mutable Account Vulnerability: target_profile と source_profile が同じアカウントであるかチェックしない
    pub fn transfer_trust_score(ctx: Context<TransferTrustScore>, score_to_transfer: u32) -> Result<()> {
        let target_profile = &mut ctx.accounts.target_profile;
        let source_profile = &mut ctx.accounts.source_profile;

        // 分岐の具体例: スコアが足りなければスコアを減らさずにメッセージを出力
        if source_profile.trust_score >= score_to_transfer {
            target_profile.trust_score = target_profile.trust_score.saturating_add(score_to_transfer);
            source_profile.trust_score = source_profile.trust_score.checked_sub(score_to_transfer).unwrap_or(0);
            msg!("Transferred {} score from source to target.", score_to_transfer);

            if target_profile.trust_score > 10000 {
                target_profile.profile_status = ProfileStatus::Maxed;
                msg!("Target profile achieved maximum reputation!");
            }
        } else {
            msg!("Insufficient score to transfer. Transfer failed.");
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
    #[msg("Insufficient score to transfer.")]
    InsufficientScore,
}
