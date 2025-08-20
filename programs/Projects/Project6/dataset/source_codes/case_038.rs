// ──────────────────────────────────────────────────────────────────────────────
// 1) tournament_core: トーナメントの核となる状態・設定
// ──────────────────────────────────────────────────────────────────────────────
use anchor_lang::prelude::*;

declare_id!("ToUrNaMeNtCoRe1111111111111111111111111");

#[program]
pub mod tournament_core {
    use super::*;

    pub fn init_tournament(
        ctx: Context<InitTournament>,
        tournament_id: u32,
        name: String,
        max_participants: u32,
        max_rounds: u32,
        prize_pool: u64,
        start_time: i64,
        end_time: i64,
    ) -> Result<()> {
        let t = &mut ctx.accounts.tournament;
        t.tournament_id = tournament_id;
        t.tournament_name = name;
        t.tournament_status = TournamentStatus::Registration;
        t.max_participants = max_participants;
        t.remaining_participants = max_participants;
        t.current_round = 0;
        t.max_rounds = max_rounds;
        t.prize_pool = prize_pool;
        t.total_prize_distributed = 0;
        t.completed_matches = 0;
        t.round_start_time = start_time;
        t.round_end_time = end_time;
        t.champion = Pubkey::default();
        Ok(())
    }

    pub fn open_tournament(ctx: Context<MutTournament>) -> Result<()> {
        let t = &mut ctx.accounts.tournament;
        // Registration → InProgress
        if let TournamentStatus::Registration = t.tournament_status {
            t.tournament_status = TournamentStatus::InProgress;
        }
        Ok(())
    }

    pub fn finalize(ctx: Context<MutTournament>) -> Result<()> {
        let t = &mut ctx.accounts.tournament;
        // Finals or InProgress → Completed（単純遷移）
        if let TournamentStatus::Finals = t.tournament_status {
            t.tournament_status = TournamentStatus::Completed;
        } else {
            if let TournamentStatus::InProgress = t.tournament_status {
                t.tournament_status = TournamentStatus::Completed;
            }
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitTournament<'info> {
    #[account(
        init,
        payer = admin,
        space = 8 + Tournament::MAX_SIZE
    )]
    pub tournament: Account<'info, Tournament>,
    #[account(mut)]
    pub admin: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct MutTournament<'info> {
    #[account(mut)]
    pub tournament: Account<'info, Tournament>,
    pub admin: Signer<'info>,
}

#[account]
pub struct Tournament {
    pub tournament_id: u32,
    pub tournament_name: String,
    pub tournament_status: TournamentStatus,
    pub max_participants: u32,
    pub remaining_participants: u32,
    pub current_round: u32,
    pub max_rounds: u32,
    pub prize_pool: u64,
    pub total_prize_distributed: u64,
    pub completed_matches: u32,
    pub round_start_time: i64,
    pub round_end_time: i64,
    pub champion: Pubkey,
}

impl Tournament {
    pub const MAX_NAME: usize = 64; // 例
    pub const MAX_SIZE: usize =
        4 + 4 + 1 + 4 + 4 + 4 + 4 + 8 + 8 + 4 + 8 + 8 + 32 + 4 + Self::MAX_NAME; // ざっくり例
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub enum TournamentStatus {
    Registration,
    InProgress,
    Finals,
    Completed,
}
