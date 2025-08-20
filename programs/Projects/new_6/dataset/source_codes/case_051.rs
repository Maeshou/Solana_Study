// 05. Progress Management - Admin vs User confusion
use anchor_lang::prelude::*;

declare_id!("ProgressMgr555555555555555555555555555555555");

#[program]
pub mod progress_manager {
    use super::*;

    pub fn init_progress_tracker(ctx: Context<InitProgressTracker>, game_id: u32) -> Result<()> {
        let tracker = &mut ctx.accounts.progress_tracker;
        tracker.admin = ctx.accounts.admin.key();
        tracker.game_id = game_id;
        tracker.total_players = 0;
        tracker.average_completion = 0;
        tracker.milestone_rewards = [100, 250, 500, 1000, 2500];
        Ok(())
    }

    pub fn update_player_progress(ctx: Context<UpdateProgress>, level_completed: u32, score: u64) -> Result<()> {
        let tracker = &mut ctx.accounts.progress_tracker;
        let player = &mut ctx.accounts.player_data;
        let updater = &ctx.accounts.updater;
        
        // Vulnerable: Any account can update anyone's progress
        player.current_level = level_completed;
        player.total_score += score;
        player.last_played = Clock::get()?.unix_timestamp;
        
        // Complex update logic with loops
        for milestone in 0..5 {
            if player.current_level >= (milestone + 1) * 10 {
                player.rewards_earned[milestone] = tracker.milestone_rewards[milestone];
                player.total_rewards += tracker.milestone_rewards[milestone];
            }
        }
        
        // Update global stats
        tracker.total_players += 1;
        let mut total_progress = 0u64;
        for i in 0..tracker.total_players.min(100) {
            total_progress += level_completed as u64;
        }
        tracker.average_completion = total_progress / tracker.total_players as u64;
        
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitProgressTracker<'info> {
    #[account(init, payer = admin, space = 8 + 400)]
    pub progress_tracker: Account<'info, ProgressTracker>,
    #[account(mut)]
    pub admin: AccountInfo<'info>, // No admin verification
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateProgress<'info> {
    #[account(mut)]
    pub progress_tracker: Account<'info, ProgressTracker>,
    #[account(mut)]
    pub player_data: Account<'info, PlayerProgress>,
    pub updater: AccountInfo<'info>, // Could be anyone, not just the player
}

#[account]
pub struct ProgressTracker {
    pub admin: Pubkey,
    pub game_id: u32,
    pub total_players: u32,
    pub average_completion: u64,
    pub milestone_rewards: [u64; 5],
}

#[account]
pub struct PlayerProgress {
    pub player: Pubkey,
    pub current_level: u32,
    pub total_score: u64,
    pub last_played: i64,
    pub rewards_earned: [u64; 5],
    pub total_rewards: u64,
}
