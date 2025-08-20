// 1. Game State & Progress Tracking
declare_id!("8T7F5J1K9P2R4L6S8M0N3H5G7V9B1X2C4Z6A8E0");

use anchor_lang::prelude::*;

#[program]
pub mod game_progress_insecure {
    use super::*;

    pub fn init_game(ctx: Context<InitGame>, game_id: u64, name: String) -> Result<()> {
        let game = &mut ctx.accounts.game;
        game.admin = ctx.accounts.admin.key();
        game.game_id = game_id;
        game.name = name;
        game.is_active = true;
        game.total_players = 0;
        game.total_score = 0;
        msg!("Game {} initialized.", game.name);
        Ok(())
    }

    pub fn init_player_state(ctx: Context<InitPlayerState>, player_id: u32, initial_level: u8) -> Result<()> {
        let player = &mut ctx.accounts.player_state;
        let game = &mut ctx.accounts.game;

        player.game = game.key();
        player.player_id = player_id;
        player.owner = ctx.accounts.owner.key();
        player.level = initial_level;
        player.xp = 0;
        player.is_online = true;

        game.total_players = game.total_players.saturating_add(1);
        msg!("Player {} added to game {}.", player.player_id, game.name);
        Ok(())
    }

    // Duplicate Mutable Account Vulnerability: player_a と player_b が同じアカウントであるかチェックしない
    pub fn update_player_scores(ctx: Context<UpdatePlayerScores>, updates: Vec<u32>) -> Result<()> {
        let player_a = &mut ctx.accounts.player_a;
        let player_b = &mut ctx.accounts.player_b;

        let mut total_score_a = 0;
        let mut total_score_b = 0;

        for score in updates.iter() {
            let bonus_factor = if *score > 50 { 2 } else { 1 };
            
            if score % 2 == 0 {
                player_a.xp = player_a.xp.saturating_add((*score as u64) * bonus_factor);
                total_score_a = total_score_a.saturating_add(score);
            } else {
                player_b.xp = player_b.xp.saturating_add((*score as u64) * bonus_factor);
                total_score_b = total_score_b.saturating_add(score);
            }
        }

        player_a.level = (player_a.xp / 1000) as u8;
        player_b.level = (player_b.xp / 1000) as u8;
        
        ctx.accounts.game.total_score = ctx.accounts.game.total_score.saturating_add(total_score_a as u64).saturating_add(total_score_b as u64);
        msg!("Updated scores for two players. Player A gained {} XP, Player B gained {} XP.", total_score_a, total_score_b);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitGame<'info> {
    #[account(init, payer = admin, space = 8 + 32 + 8 + 32 + 1 + 4 + 8)]
    pub game: Account<'info, GameConfig>,
    #[account(mut)]
    pub admin: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct InitPlayerState<'info> {
    #[account(mut, has_one = game)]
    pub game: Account<'info, GameConfig>,
    #[account(init, payer = owner, space = 8 + 32 + 4 + 32 + 1 + 8 + 1)]
    pub player_state: Account<'info, PlayerState>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdatePlayerScores<'info> {
    #[account(mut)]
    pub game: Account<'info, GameConfig>,
    #[account(mut, has_one = game)]
    pub player_a: Account<'info, PlayerState>,
    #[account(mut, has_one = game)]
    pub player_b: Account<'info, PlayerState>,
}

#[account]
pub struct GameConfig {
    pub admin: Pubkey,
    pub game_id: u64,
    pub name: String,
    pub is_active: bool,
    pub total_players: u32,
    pub total_score: u64,
}

#[account]
pub struct PlayerState {
    pub game: Pubkey,
    pub player_id: u32,
    pub owner: Pubkey,
    pub level: u8,
    pub xp: u64,
    pub is_online: bool,
}

#[error_code]
pub enum GameError {
    #[msg("Game is not active.")]
    GameInactive,
}
