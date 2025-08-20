// ──────────────────────────────────────────────────────────────────────────────
// 3) bracket_engine: ブラケット進行（advance_tournament_bracket の分離）
//   - 元のadvanceロジックを自己完結させた版
//   - match / else if を使わない形に調整（しきい値は多段 if）
// ──────────────────────────────────────────────────────────────────────────────
use anchor_lang::prelude::*;

declare_id!("BrAcKeTeNgInE1111111111111111111111111");

#[program]
pub mod bracket_engine {
    use super::*;

    pub fn advance_tournament_bracket(ctx: Context<AdvanceTournament>) -> Result<()> {
        let tournament = &mut ctx.accounts.tournament;
        let winner_profile = &mut ctx.accounts.winner_profile;
        let loser_profile = &mut ctx.accounts.loser_profile;

        // 進行状態（== を避けたい場合は段階的遷移のみ通す方針でチェック）
        // ここでは単純に InProgress が前提で進める例（外側で set 済み前提）
        require!(
            matches!(tournament.tournament_status, TournamentStatus::InProgress),
            TournamentError::TournamentNotActive
        );

        require!(tournament.current_round < tournament.max_rounds, TournamentError::TournamentCompleted);

        let now = Clock::get()?.unix_timestamp;
        require!(now >= tournament.round_start_time, TournamentError::RoundNotStarted);
        require!(now <= tournament.round_end_time, TournamentError::RoundExpired);

        // ── 戦闘力計算（match回避：係数テーブル）
        let mut winner_power: u64 = 0;
        let mut loser_power: u64 = 0;
        let multipliers: [u64; 3] = [150, 100, 75];

        // winner 装備
        let mut wi = 0usize;
        while wi < winner_profile.equipped_items.len() {
            let e = &winner_profile.equipped_items[wi];
            let idx = e.kind_index();
            let m = multipliers[idx];
            winner_power = winner_power
                .checked_add(e.power_level.saturating_mul(m))
                .unwrap();
            wi += 1;
        }
        // loser 装備
        let mut li = 0usize;
        while li < loser_profile.equipped_items.len() {
            let e = &loser_profile.equipped_items[li];
            let idx = e.kind_index();
            let m = multipliers[idx];
            loser_power = loser_power
                .checked_add(e.power_level.saturating_mul(m))
                .unwrap();
            li += 1;
        }

        // スキルボーナス
        winner_power = winner_power.checked_add(winner_profile.skill_level.saturating_mul(200)).unwrap();
        loser_power  = loser_power.checked_add(loser_profile.skill_level.saturating_mul(200)).unwrap();

        // 勝敗差
        let diff = winner_power.saturating_sub(loser_power);

        // 勝利種別（多段 if）
        let mut margin = VictoryType::Narrow;
        if diff > 500 { margin = VictoryType::Clear; }
        if diff > 1000 { margin = VictoryType::Decisive; }

        // 報酬
        let base_reward = tournament.prize_pool / (tournament.remaining_participants as u64);
        let mut victory_bonus = base_reward;
        if let VictoryType::Clear = margin {
            victory_bonus = base_reward.checked_mul(125).unwrap() / 100;
        }
        if let VictoryType::Decisive = margin {
            victory_bonus = base_reward.checked_mul(150).unwrap() / 100;
        }

        let round_multiplier = tournament.current_round.checked_add(1).unwrap();
        let final_reward = victory_bonus.checked_mul(round_multiplier as u64).unwrap();

        winner_profile.tournament_earnings = winner_profile.tournament_earnings.checked_add(final_reward).unwrap();
        winner_profile.wins = winner_profile.wins.saturating_add(1);
        winner_profile.current_win_streak = winner_profile.current_win_streak.saturating_add(1);

        loser_profile.losses = loser_profile.losses.saturating_add(1);
        loser_profile.current_win_streak = 0;
        loser_profile.elimination_round = tournament.current_round;

        // 連勝ボーナス
        if winner_profile.current_win_streak >= 5 {
            let s = winner_profile.current_win_streak.saturating_mul(100);
            winner_profile.tournament_earnings = winner_profile.tournament_earnings.saturating_add(s as u64);
        }

        // 進行更新
        tournament.remaining_participants = tournament.remaining_participants.saturating_sub(1);
        tournament.completed_matches = tournament.completed_matches.saturating_add(1);
        tournament.total_prize_distributed = tournament.total_prize_distributed.saturating_add(final_reward);

        // ラウンド終了処理
        let matches_per_round = tournament.remaining_participants / 2;
        if tournament.completed_matches >= matches_per_round {
            tournament.current_round = tournament.current_round.saturating_add(1);
            tournament.completed_matches = 0;
            tournament.round_start_time = now + 1800;
            tournament.round_end_time = tournament.round_start_time + 3600;
        }

        // 決勝処理
        if tournament.remaining_participants <= 2 {
            if tournament.current_round >= tournament.max_rounds {
                tournament.tournament_status = TournamentStatus::Finals;
                tournament.champion = winner_profile.player;
                let champion_bonus = tournament.prize_pool / 4;
                winner_profile.tournament_earnings = winner_profile.tournament_earnings.saturating_add(champion_bonus);
                winner_profile.championships_won = winner_profile.championships_won.saturating_add(1);
            }
        }

        emit!(MatchCompleted {
            tournament_id: tournament.tournament_id,
            winner: winner_profile.player,
            loser: loser_profile.player,
            round: tournament.current_round,
            victory_type: margin.clone(),
            reward_earned: final_reward,
        });

        Ok(())
    }
}

#[derive(Accounts)]
pub struct AdvanceTournament<'info> {
    #[account(mut)]
    pub tournament: Account<'info, Tournament>,
    #[account(mut)]
    pub winner_profile: Account<'info, PlayerProfile>,
    #[account(mut)]
    pub loser_profile: Account<'info, PlayerProfile>,
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

// ── 型（自己完結用に複製）
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

#[account]
pub struct PlayerProfile {
    pub player: Pubkey,
    pub player_name: String,
    pub skill_level: u64,
    pub wins: u32,
    pub losses: u32,
    pub current_win_streak: u32,
    pub championships_won: u32,
    pub tournament_earnings: u64,
    pub elimination_round: u32,
    pub equipped_items: Vec<Equipment>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct Equipment {
    pub item_id: u32,
    pub equipment_type: EquipmentType,
    pub power_level: u64,
    pub enhancement_level: u32,
}
impl Equipment {
    pub fn kind_index(&self) -> usize {
        // 0:Weapon, 1:Armor, 2:Accessory
        if let EquipmentType::Weapon = self.equipment_type { return 0; }
        if let EquipmentType::Armor = self.equipment_type { return 1; }
        2
    }
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub enum TournamentStatus { Registration, InProgress, Finals, Completed }

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub enum EquipmentType { Weapon, Armor, Accessory }

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub enum VictoryType { Narrow, Clear, Decisive }

#[error_code]
pub enum TournamentError {
    #[msg("Tournament is not active")] TournamentNotActive,
    #[msg("Tournament has been completed")] TournamentCompleted,
    #[msg("Round has not started yet")] RoundNotStarted,
    #[msg("Round time has expired")] RoundExpired,
}
