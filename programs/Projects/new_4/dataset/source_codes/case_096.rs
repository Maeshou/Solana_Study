use anchor_lang::prelude::*;

declare_id!("VulnInit5555555555555555555555555555555555");

#[program]
pub mod vuln_game {
    use super::*;

    pub fn init_game(ctx: Context<InitGame>, max_players: u8) -> Result<()> {
        let session = &mut ctx.accounts.session;     // ← Init OK
        session.max_players = max_players;
        session.active      = true;

        let board = &mut ctx.accounts.board;         // ← Init OK
        board.scores = Vec::new();
        for _ in 0..max_players {
            board.scores.push(0);
        }

        // game_log は init されていない → 任意差し替え
        let game_log = &mut ctx.accounts.game_log;   // ← Init missing
        game_log.events = Vec::new();
        game_log.events.push("Game start".to_string());
        if max_players > 5 {
            game_log.events.push("Large game".to_string());
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitGame<'info> {
    #[account(init, payer = host, space = 8 + 1 + 1)]
    pub session: Account<'info, SessionData>,
    #[account(init, payer = host, space = 8 + 4 + (4 * 10))]
    pub board: Account<'info, BoardData>,
    pub game_log: Account<'info, GameLogData>,    // ← init がない
    #[account(mut)] pub host: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct SessionData {
    pub max_players: u8,
    pub active: bool,
}

#[account]
pub struct BoardData {
    pub scores: Vec<u32>,
}

#[account]
pub struct GameLogData {
    pub events: Vec<String>,
}
