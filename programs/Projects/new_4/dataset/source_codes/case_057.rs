// 2. ゲームセッション＋スコア履歴
use anchor_lang::prelude::*;
declare_id!("GAMES111122223333444455556666777788");

#[program]
pub mod misinit_game_v7 {
    use super::*;

    pub fn start_session(
        ctx: Context<StartSession>,
        session_id: u64,
    ) -> Result<()> {
        let s = &mut ctx.accounts.session;
        s.session_id = session_id;
        s.active = true;
        Ok(())
    }

    pub fn end_session(ctx: Context<StartSession>) -> Result<()> {
        let s = &mut ctx.accounts.session;
        s.active = false;
        Ok(())
    }

    pub fn record_score(
        ctx: Context<StartSession>,
        player: Pubkey,
        score: u32,
    ) -> Result<()> {
        require!(score <= 1_000_000, ErrorCode2::ExcessiveScore);
        let hist = &mut ctx.accounts.score_history;
        hist.entries.push((player, score));
        Ok(())
    }
}

#[derive(Accounts)]
pub struct StartSession<'info> {
    #[account(init, payer = admin, space = 8 + 8 + 1)] pub session: Account<'info, SessionData>,
    #[account(mut)] pub score_history: Account<'info, ScoreHistory>,
    #[account(mut)] pub admin: Signer<'info>, pub system_program: Program<'info, System>,
}

#[account]
pub struct SessionData { pub session_id: u64, pub active: bool }
#[account]
pub struct ScoreHistory { pub entries: Vec<(Pubkey,u32)> }

#[error_code]
pub enum ErrorCode2 { #[msg("スコアが大きすぎます。")] ExcessiveScore }
