// 2. リーダーボード＋スコアログ
use anchor_lang::prelude::*;
declare_id!("LEADAAAABBBBCCCCDDDDEEEEFFFF9999");

#[program]
pub mod misinit_leaderboard_v6 {
    use super::*;

    pub fn init_leaderboard(
        ctx: Context<InitLeaderboard>,
        max_players: u16,
    ) -> Result<()> {
        let lb = &mut ctx.accounts.leaderboard;
        lb.max_players = max_players;
        lb.current_count = 0;
        lb.top_scores = Vec::new();
        Ok(())
    }

    pub fn add_score(
        ctx: Context<InitLeaderboard>,
        player: Pubkey,
        score: u32,
    ) -> Result<()> {
        require!(score > 0, ErrorCode2::ZeroScore);
        let lb = &mut ctx.accounts.leaderboard;
        if lb.current_count < lb.max_players {
            lb.current_count += 1;
        }
        lb.top_scores.push((player, score));
        Ok(())
    }

    pub fn log_score(
        ctx: Context<InitLeaderboard>,
        player: Pubkey,
    ) -> Result<()> {
        let log = &mut ctx.accounts.score_log;
        log.players.push(player);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitLeaderboard<'info> {
    #[account(init, payer = admin, space = 8 + 2 + 2 + (4 + 32*10))]
    pub leaderboard: Account<'info, LeaderboardData>,
    #[account(mut)] pub score_log: Account<'info, ScoreLog>,
    #[account(mut)] pub admin: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct LeaderboardData { pub max_players: u16, pub current_count: u16, pub top_scores: Vec<(Pubkey,u32)> }
#[account]
pub struct ScoreLog { pub players: Vec<Pubkey> }

#[error_code]
pub enum ErrorCode2 { #[msg("スコアは0より大きくなければなりません。")] ZeroScore }

