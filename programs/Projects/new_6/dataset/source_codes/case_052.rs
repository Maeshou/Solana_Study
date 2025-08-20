// 06. Tournament System - Organizer vs Participant confusion
use anchor_lang::prelude::*;

declare_id!("Tournament66666666666666666666666666666666666");

#[program]
pub mod tournament_system {
    use super::*;

    pub fn init_tournament(ctx: Context<InitTournament>, entry_fee: u64, max_participants: u32) -> Result<()> {
        let tournament = &mut ctx.accounts.tournament;
        tournament.organizer = ctx.accounts.organizer.key();
        tournament.entry_fee = entry_fee;
        tournament.max_participants = max_participants;
        tournament.current_participants = 0;
        tournament.prize_pool = 0;
        tournament.tournament_status = 0; // 0: Open, 1: Active, 2: Ended
        tournament.round_number = 1;
        Ok(())
    }

    pub fn advance_tournament(ctx: Context<AdvanceTournament>, eliminated_players: Vec<Pubkey>) -> Result<()> {
        let tournament = &mut ctx.accounts.tournament;
        let controller = &ctx.accounts.controller;
        
        // Vulnerable: Any account can control tournament flow
        tournament.round_number += 1;
        
        // Complex elimination logic
        for eliminated in eliminated_players.iter() {
            for i in 0..tournament.current_participants {
                if tournament.participants[i as usize] == *eliminated {
                    tournament.participants[i as usize] = Pubkey::default();
                    tournament.current_participants -= 1;
                    break;
                }
            }
        }
        
        // Update prize distribution
        if tournament.current_participants <= 3 {
            tournament.tournament_status = 2; // Ended
            let winner_prize = tournament.prize_pool * 60 / 100;
            let second_prize = tournament.prize_pool * 30 / 100;
            let third_prize = tournament.prize_pool * 10 / 100;
            
            tournament.prize_distribution = [winner_prize, second_prize, third_prize];
        }
        
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitTournament<'info> {
    #[account(init, payer = organizer, space = 8 + 2000)]
    pub tournament: Account<'info, TournamentData>,
    #[account(mut)]
    pub organizer: AccountInfo<'info>, // No organizer role verification
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct AdvanceTournament<'info> {
    #[account(mut)]
    pub tournament: Account<'info, TournamentData>,
    pub controller: AccountInfo<'info>, // Could be anyone, not just organizer
}

#[account]
pub struct TournamentData {
    pub organizer: Pubkey,
    pub entry_fee: u64,
    pub max_participants: u32,
    pub current_participants: u32,
    pub prize_pool: u64,
    pub tournament_status: u8,
    pub round_number: u32,
    pub participants: [Pubkey; 100],
    pub prize_distribution: [u64; 3],
}