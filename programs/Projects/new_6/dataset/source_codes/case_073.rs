// 24. Gaming Tournament - Organizer vs Player confusion
use anchor_lang::prelude::*;

declare_id!("GamingTournament444444444444444444444444444444");

#[program]
pub mod gaming_tournament {
    use super::*;

    pub fn init_tournament_bracket(ctx: Context<InitTournamentBracket>, game_type: u8, max_players: u32, entry_fee: u64) -> Result<()> {
        let bracket = &mut ctx.accounts.tournament_bracket;
        bracket.organizer = ctx.accounts.organizer.key();
        bracket.game_type = game_type;
        bracket.max_players = max_players;
        bracket.entry_fee = entry_fee;
        bracket.current_round = 1;
        bracket.tournament_status = 0; // Registration
        bracket.total_prize_pool = 0;
        bracket.registered_players = 0;
        bracket.eliminated_players = 0;
        bracket.start_time = 0;
        bracket.registration_deadline = Clock::get()?.unix_timestamp + 86400; // 24 hours
        Ok(())
    }

    pub fn manage_tournament_flow(ctx: Context<ManageTournamentFlow>, action: u8, player_results: Vec<(Pubkey, u32, bool)>) -> Result<()> {
        let bracket = &mut ctx.accounts.tournament_bracket;
        let match_data = &mut ctx.accounts.match_data;
        let manager = &ctx.accounts.manager;
        
        // Vulnerable: Any account can manage tournament flow
        let current_time = Clock::get()?.unix_timestamp;
        
        match action {
            0 => { // Start tournament
                if bracket.registered_players >= 4 {
                    bracket.tournament_status = 1; // Active
                    bracket.start_time = current_time;
                    
                    // Initialize first round matches
                    let matches_in_round = bracket.registered_players / 2;
                    match_data.current_round_matches = matches_in_round;
                    match_data.matches_completed = 0;
                }
            },
            1 => { // Process match results
                for (i, (player, score, winner)) in player_results.iter().enumerate().take(16) {
                    match_data.match_results[i] = MatchResult {
                        player: *player,
                        score: *score,
                        is_winner: *winner,
                        match_time: current_time,
                    };
                    
                    if *winner {
                        match_data.round_winners[match_data.matches_completed as usize] = *player;
                    } else {
                        bracket.eliminated_players += 1;
                        
                        // Consolation prize calculation
                        let consolation = (bracket.total_prize_pool * 5) / 100; // 5%
                        match_data.consolation_prizes[i] = consolation;
                    }
                }
                
                match_data.matches_completed += player_results.len() as u32;
            },
            2 => { // Advance round
                if match_data.matches_completed >= match_data.current_round_matches {
                    bracket.current_round += 1;
                    let remaining_players = bracket.registered_players - bracket.eliminated_players;
                    
                    // Check if tournament is complete
                    if remaining_players <= 1 {
                        bracket.tournament_status = 3; // Completed
                        bracket.end_time = current_time;
                        
                        // Distribute final prizes
                        let winner_prize = (bracket.total_prize_pool * 60) / 100;
                        let runner_up_prize = (bracket.total_prize_pool * 25) / 100;
                        let third_place_prize = (bracket.total_prize_pool * 15) / 100;
                        
                        match_data.final_prizes = [winner_prize, runner_up_prize, third_place_prize];
                    } else {
                        // Setup next round
                        match_data.current_round_matches = remaining_players / 2;
                        match_data.matches_completed = 0;
                        
                        // Copy winners to next round participants
                        for i in 0..remaining_players.min(32) {
                            bracket.active_players[i as usize] = match_data.round_winners[i as usize];
                        }
                    }
                }
            },
            3 => { // Emergency pause
                bracket.tournament_status = 4; // Paused
                bracket.pause_reason = if player_results.len() > 0 { 1 } else { 2 };
                bracket.paused_at = current_time;
            },
            _ => {}
        }
        
        // Update player statistics
        for (player, score, winner) in player_results.iter() {
            if let Some(player_index) = bracket.active_players.iter().position(|&p| p == *player) {
                bracket.player_scores[player_index] += *score;
                if *winner {
                    bracket.player_wins[player_index] += 1;
                }
                bracket.player_matches[player_index] += 1;
            }
        }
        
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitTournamentBracket<'info> {
    #[account(init, payer = organizer, space = 8 + 1200)]
    pub tournament_bracket: Account<'info, TournamentBracketData>,
    #[account(mut)]
    pub organizer: AccountInfo<'info>, // No organizer verification
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ManageTournamentFlow<'info> {
    #[account(mut)]
    pub tournament_bracket: Account<'info, TournamentBracketData>,
    #[account(mut)]
    pub match_data: Account<'info, MatchData>,
    pub manager: AccountInfo<'info>, // Could be anyone, not just organizer
}

#[derive(Debug, Clone, Copy, AnchorSerialize, AnchorDeserialize)]
pub struct MatchResult {
    pub player: Pubkey,
    pub score: u32,
    pub is_winner: bool,
    pub match_time: i64,
}

impl Default for MatchResult {
    fn default() -> Self {
        Self {
            player: Pubkey::default(),
            score: 0,
            is_winner: false,
            match_time: 0,
        }
    }
}

#[account]
pub struct TournamentBracketData {
    pub organizer: Pubkey,
    pub game_type: u8,
    pub max_players: u32,
    pub entry_fee: u64,
    pub current_round: u32,
    pub tournament_status: u8,
    pub total_prize_pool: u64,
    pub registered_players: u32,
    pub eliminated_players: u32,
    pub start_time: i64,
    pub end_time: i64,
    pub registration_deadline: i64,
    pub paused_at: i64,
    pub pause_reason: u8,
    pub active_players: [Pubkey; 32],
    pub player_scores: [u32; 32],
    pub player_wins: [u32; 32],
    pub player_matches: [u32; 32],
}

#[account]
pub struct MatchData {
    pub current_round_matches: u32,
    pub matches_completed: u32,
    pub match_results: [MatchResult; 16],
    pub round_winners: [Pubkey; 16],
    pub consolation_prizes: [u64; 16],
    pub final_prizes: [u64; 3],
}
