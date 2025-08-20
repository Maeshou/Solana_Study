use anchor_lang::prelude::*;

declare_id!("SafeMulti5555555555555555555555555555555555");

#[program]
pub mod safe_game {
    use super::*;

    // session, board, game_log をすべて初期化
    pub fn init_game(
        ctx: Context<InitGame>,
        max_players: u8,
    ) -> Result<()> {
        let session = &mut ctx.accounts.session;
        session.max_players = max_players;
        session.active = true;

        let board = &mut ctx.accounts.board;
        board.scores = Vec::new();
        for _ in 0..max_players {
            board.scores.push(0);
        }

        let game_log = &mut ctx.accounts.game_log;
        game_log.events = Vec::new();
        game_log.events.push("Game started".to_string());
        Ok(())
    }

    // board, session, game_log を mut 更新
    pub fn play_round(
        ctx: Context<PlayRound>,
        player_index: usize,
        score: u32,
    ) -> Result<()> {
        let session = &ctx.accounts.session;
        let board = &mut ctx.accounts.board;
        let log   = &mut ctx.accounts.game_log;

        if player_index < session.max_players as usize {
            board.scores[player_index] += score;
            log.events.push(format!("Player {} scored {}", player_index, score));
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
    #[account(init, payer = host, space = 8 + 4 + (200 * 2))]
    pub game_log: Account<'info, GameLogData>,
    #[account(mut)] pub host: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct PlayRound<'info> {
    pub session: Account<'info, SessionData>,
    #[account(mut)] pub board: Account<'info, BoardData>,
    #[account(mut)] pub game_log: Account<'info, GameLogData>,
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
