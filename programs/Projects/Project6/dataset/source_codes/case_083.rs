// 1) tournament_core: トーナメントの核（軽量）
use anchor_lang::prelude::*;

declare_id!("ToUrNaMeNtCoRe1111111111111111111111111");

#[program]
pub mod tournament_core {
    use super::*;

    pub fn init_tournament(
        ctx: Context<InitTournament>,
        tournament_id: u32,
        name: String,
        max_rounds: u32,
        prize_pool: u64,
        start_time: i64,
        end_time: i64,
    ) -> Result<()> {
        let t = &mut ctx.accounts.tournament;
        t.tournament_id = tournament_id;
        t.tournament_name = name;
        t.tournament_status = TournamentStatus::Registration;
        t.current_round = 0;
        t.max_rounds = max_rounds;
        t.prize_pool = prize_pool;
        t.round_start_time = start_time;
        t.round_end_time = end_time;
        t.champion = Pubkey::default();
        Ok(())
    }

    pub fn start(ctx: Context<MutTournament>) -> Result<()> {
        let t = &mut ctx.accounts.tournament;
        // 単純遷移：Registration から進める前提
        t.tournament_status = TournamentStatus::InProgress;
        Ok(())
    }

    pub fn complete(ctx: Context<MutTournament>) -> Result<()> {
        let t = &mut ctx.accounts.tournament;
        // 決着
        t.tournament_status = TournamentStatus::Completed;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitTournament<'info> {
    #[account(init, payer = admin, space = 8 + Tournament::MAX_SIZE)]
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
    pub current_round: u32,
    pub max_rounds: u32,
    pub prize_pool: u64,
    pub round_start_time: i64,
    pub round_end_time: i64,
    pub champion: Pubkey,
}

impl Tournament {
    pub const MAX_NAME: usize = 64;
    pub const MAX_SIZE: usize =
        4 + 4 + Self::MAX_NAME + 1 + 4 + 4 + 8 + 8 + 8 + 32;
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub enum TournamentStatus { Registration, InProgress, Finals, Completed }
