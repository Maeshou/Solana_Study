
// 7. Game Arena
declare_id!("GA77777777777777777777777777777777");
use anchor_lang::prelude::*;

#[program]
pub mod game_arena {
    use super::*;
    pub fn init_game(ctx: Context<InitGame>, difficulty: u32) -> Result<()> {
        ctx.accounts.game_config.max_rounds = 10;
        ctx.accounts.game_config.difficulty = difficulty;
        ctx.accounts.game_config.bump = *ctx.bumps.get("game_config").unwrap();
        ctx.accounts.player_stats.score = 0;
        ctx.accounts.player_stats.level = 1;
        ctx.accounts.player_stats.active = true;
        ctx.accounts.score_board.total_score = 0;
        ctx.accounts.score_board.plays = 0;
        ctx.accounts.score_board.locked = false;
        Ok(())
    }
    pub fn play_round(ctx: Context<PlayRound>, points: u8) -> Result<()> {
        let mut total = ctx.accounts.score_board.total_score;
        for _ in 0..points {
            total += ctx.accounts.player_stats.score;
        }
        require!(
            ctx.accounts.game_config.key() != ctx.accounts.score_board.key(),
            ProgramError::InvalidArgument
        );
        if total > ctx.accounts.game_config.difficulty as u64 {
            ctx.accounts.player_stats.level += 1;
            msg!("Level up: {}", ctx.accounts.player_stats.level);
            ctx.accounts.score_board.locked = true;
            ctx.accounts.score_board.plays += 1;
        } else {
            ctx.accounts.player_stats.level = 0;
            msg!("Reset level");
            ctx.accounts.score_board.locked = false;
            ctx.accounts.score_board.plays -= 1;
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitGame<'info> {
    #[account(init, payer = payer, space = 8 + 1 + 4 + 1)]
    pub game_config: Account<'info, GameConfig>,
    #[account(init, payer = payer, space = 8 + 8 + 4 + 1)]
    pub player_stats: Account<'info, PlayerStats>,
    #[account(init, payer = payer, space = 8 + 8 + 4 + 1)]
    pub score_board: Account<'info, ScoreBoard>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct PlayRound<'info> {
    #[account(mut, constraint = game_config.key() != player_stats.key(), error = GameError::SameAccount)]
    pub game_config: Account<'info, GameConfig>,
    #[account(mut)]
    pub player_stats: Account<'info, PlayerStats>,
    #[account(mut)]
    pub score_board: Account<'info, ScoreBoard>,
    pub player: Signer<'info>,
}

#[account]
pub struct GameConfig {
    pub max_rounds: u8,
    pub difficulty: u32,
    pub bump: u8,
}

#[account]
pub struct PlayerStats {
    pub score: u64,
    pub level: u32,
    pub active: bool,
}

#[account]
pub struct ScoreBoard {
    pub total_score: u64,
    pub plays: u32,
    pub locked: bool,
}

#[error_code]
pub enum GameError {
    #[msg("Duplicate accounts")]
    SameAccount,
}
