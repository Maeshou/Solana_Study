use anchor_lang::prelude::*;

declare_id!("InitAll5555555555555555555555555555555555");

#[program]
pub mod multi_init5 {
    use super::*;

    // セッション、スコアボード、ログをループ・分岐で初期化
    pub fn init_game(ctx: Context<InitGame>, max_players: u8) -> Result<()> {
        let session = &mut ctx.accounts.session;
        session.max_players = max_players;
        session.active = true;

        // board.scores に 0 を max_players 回 push
        let board = &mut ctx.accounts.board;
        board.scores = Vec::new();
        for _ in 0..max_players {
            board.scores.push(0);
        }

        // ログに初期メッセージ＋規模に応じた分岐メッセージ
        let game_log = &mut ctx.accounts.game_log;
        game_log.events = Vec::new();
        game_log.events.push("Game initialized".to_string());
        if max_players > 5 {
            game_log.events.push("Large session".to_string());
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
