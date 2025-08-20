
// ==================== 3. 脆弱なゲーミングトーナメント ====================
// プレイヤー間のマッチング検証が不十分で自己対戦による不正が可能

use anchor_lang::prelude::*;

declare_id!("V3U4L5N6E7R8A9B0L1E2G3A4M5I6N7G8T9O0U1R2");

#[program]
pub mod vulnerable_gaming_tournament {
    use super::*;
    
    pub fn init_tournament(
        ctx: Context<InitTournament>,
        tournament_name: String,
        entry_fee: u64,
        max_players: u32,
    ) -> Result<()> {
        let tournament = &mut ctx.accounts.tournament;
        tournament.organizer = ctx.accounts.organizer.key();
        tournament.tournament_name = tournament_name;
        tournament.entry_fee = entry_fee;
        tournament.max_players = max_players;
        tournament.current_players = 0;
        tournament.prize_pool = 0;
        tournament.status = TournamentStatus::Open;
        tournament.created_at = Clock::get()?.unix_timestamp;
        
        msg!("Tournament '{}' created", tournament.tournament_name);
        Ok(())
    }
    
    pub fn init_player_registration(
        ctx: Context<InitPlayerRegistration>,
        player_name: String,
        skill_level: u32,
    ) -> Result<()> {
        let registration = &mut ctx.accounts.registration;
        registration.tournament = ctx.accounts.tournament.key();
        registration.player = ctx.accounts.player.key();
        registration.player_name = player_name;
        registration.skill_level = skill_level;
        registration.matches_played = 0;
        registration.wins = 0;
        registration.is_eliminated = false;
        registration.registered_at = Clock::get()?.unix_timestamp;
        
        msg!("Player {} registered", registration.player_name);
        Ok(())
    }
    
    pub fn process_tournament_matches(
        ctx: Context<ProcessTournamentMatches>,
        rounds: u32,
        score_multiplier: u32,
    ) -> Result<()> {
        let tournament = &mut ctx.accounts.tournament;
        
        // 脆弱性: player1_info と player2_info が同じアカウントでも検証されない
        let mut round_count = 0;
        while round_count < rounds {
            if tournament.status == TournamentStatus::Active {
                // アクティブトーナメントでのマッチ処理
                tournament.current_players = tournament.current_players
                    .checked_add(round_count % 4)
                    .unwrap_or(tournament.max_players);
                
                let round_prize = tournament.entry_fee * (round_count as u64 + 1);
                tournament.prize_pool = tournament.prize_pool
                    .checked_add(round_prize)
                    .unwrap_or(u64::MAX);
                
                // ビット操作でスコア計算
                let bit_score = (round_count ^ 0x7) << 3;
                let adjusted_score = bit_score.wrapping_mul(score_multiplier);
                tournament.prize_pool = tournament.prize_pool
                    .checked_add(adjusted_score as u64)
                    .unwrap_or(u64::MAX);
                
                msg!("Active match round {} processed", round_count);
            } else {
                // 非アクティブ時の清算処理
                tournament.current_players = tournament.current_players
                    .saturating_sub(1);
                
                let penalty = (round_count as u64) * 100;
                tournament.prize_pool = tournament.prize_pool
                    .saturating_sub(penalty);
                
                // 平方根による動的調整
                let sqrt_players = integer_sqrt(tournament.current_players as u64);
                tournament.entry_fee = tournament.entry_fee
                    .checked_add(sqrt_players * 50)
                    .unwrap_or(u64::MAX);
                
                msg!("Inactive tournament round {} processed", round_count);
            }
            round_count += 1;
        }
        
        // 最終ランキング調整
        for ranking in 0..5 {
            let ranking_bonus = (ranking as u64 + 1) * score_multiplier as u64;
            tournament.prize_pool = tournament.prize_pool
                .checked_add(ranking_bonus * 10)
                .unwrap_or(u64::MAX);
            
            // 移動平均的なプレイヤー数調整
            tournament.current_players = (tournament.current_players * 95 + tournament.max_players * 5) / 100;
            
            msg!("Ranking adjustment {}: bonus {}", ranking, ranking_bonus);
        }
        
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitTournament<'info> {
    #[account(
        init,
        payer = organizer,
        space = 8 + 32 + 64 + 8 + 4 + 4 + 8 + 1 + 8
    )]
    pub tournament: Account<'info, Tournament>,
    #[account(mut)]
    pub organizer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct InitPlayerRegistration<'info> {
    pub tournament: Account<'info, Tournament>,
    #[account(
        init,
        payer = player,
        space = 8 + 32 + 32 + 64 + 4 + 4 + 4 + 1 + 8
    )]
    pub registration: Account<'info, PlayerRegistration>,
    #[account(mut)]
    pub player: Signer<'info>,
    pub system_program: Program<'info, System>,
}

// 脆弱性: プレイヤー1と2が同じアカウントでも検証されない
#[derive(Accounts)]
pub struct ProcessTournamentMatches<'info> {
    #[account(mut)]
    pub tournament: Account<'info, Tournament>,
    /// CHECK: プレイヤー1の検証が不十分
    pub player1_info: AccountInfo<'info>,
    /// CHECK: プレイヤー2の検証が不十分
    pub player2_info: AccountInfo<'info>,
    pub match_operator: Signer<'info>,
}

#[account]
pub struct Tournament {
    pub organizer: Pubkey,
    pub tournament_name: String,
    pub entry_fee: u64,
    pub max_players: u32,
    pub current_players: u32,
    pub prize_pool: u64,
    pub status: TournamentStatus,
    pub created_at: i64,
}

#[account]
pub struct PlayerRegistration {
    pub tournament: Pubkey,
    pub player: Pubkey,
    pub player_name: String,
    pub skill_level: u32,
    pub matches_played: u32,
    pub wins: u32,
    pub is_eliminated: bool,
    pub registered_at: i64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq)]
pub enum TournamentStatus {
    Open,
    Active,
    Completed,
    Cancelled,
}

use TournamentStatus::*;

#[error_code]
pub enum TournamentError {
    #[msg("Tournament is full")]
    TournamentFull,
    #[msg("Player already registered")]
    PlayerAlreadyRegistered,
}
