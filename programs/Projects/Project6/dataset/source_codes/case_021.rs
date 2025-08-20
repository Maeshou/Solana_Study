// 5) events_hub: ラウンド時刻とイベント（軽量）
use anchor_lang::prelude::*;

declare_id!("EvEnTsHuB11111111111111111111111111111");

#[program]
pub mod events_hub {
    use super::*;

    pub fn schedule_next_round(ctx: Context<MutTournament>, start_in: i64, round_len: i64) -> Result<()> {
        let t = &mut ctx.accounts.tournament;
        let now = Clock::get()?.unix_timestamp;

        let mut s = now + start_in;
        let mut e = s + round_len;
        if start_in < 0 { s = now + 600; }
        if round_len < 1800 { e = s + 1800; }
        if round_len > 10_800 { e = s + 10_800; }

        t.round_start_time = s;
        t.round_end_time = e;
        Ok(())
    }

    pub fn log_match(ctx: Context<LogMatch>, a: LogArgs) -> Result<()> {
        emit!(MatchCompleted {
            tournament_id: a.tournament_id,
            winner: a.winner,
            loser: a.loser,
            round: a.round,
            victory_type: a.victory_type,
            reward_earned: a.reward_earned,
        });
        Ok(())
    }
}

#[derive(Accounts)]
pub struct MutTournament<'info> {
    #[account(mut)]
    pub tournament: Account<'info, Tournament>,
}
#[derive(Accounts)]
pub struct LogMatch<'info> {
    pub authority: Signer<'info>,
}

#[event]
pub struct MatchCompleted {
    pub tournament_id: u32,
    pub winner: Pubkey,
    pub loser: Pubkey,
    pub round: u32,
    pub victory_type: VictoryType,
    pub reward_earned: u64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct LogArgs {
    pub tournament_id: u32,
    pub winner: Pubkey,
    pub loser: Pubkey,
    pub round: u32,
    pub victory_type: VictoryType,
    pub reward_earned: u64,
}

// 最小限の型
#[account]
pub struct Tournament {
    pub tournament_id: u32,
    pub tournament_name: String,
    pub tournament_status: TournamentStatus,
    pub current_round: u32,
    pub max_rounds: u32,
    pub prize_pool: u64,
    pub round_start_time: i64,
    pub round_end_time: i64,
    pub champion: Pubkey,
}
#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub enum TournamentStatus { Registration, InProgress, Finals, Completed }
#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub enum VictoryType { Narrow, Clear, Decisive }
