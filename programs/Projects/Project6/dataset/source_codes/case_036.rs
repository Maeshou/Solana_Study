// 3) bracket_engine: ブラケット進行（軽量）
// 余分な集計を削り、判定・報酬・ラウンド更新に必要最小限だけ保持
use anchor_lang::prelude::*;

declare_id!("BrAcKeTeNgInE1111111111111111111111111");

#[program]
pub mod bracket_engine {
    use super::*;

    pub fn advance_tournament_bracket(ctx: Context<AdvanceTournament>) -> Result<()> {
        let t = &mut ctx.accounts.tournament;
        let wp = &mut ctx.accounts.winner_profile;
        let lp = &mut ctx.accounts.loser_profile;

        // ラウンド時間の基本整合のみ（状態は外部で管理）
        let now = Clock::get()?.unix_timestamp;
        require!(now >= t.round_start_time, BracketErr::RoundNotStarted);
        require!(now <= t.round_end_time, BracketErr::RoundExpired);
        require!(t.current_round < t.max_rounds, BracketErr::TournamentCompleted);

        // 装備係数（match不使用）
        let mul = [150u64, 100, 75];

        let mut w_power = 0u64;
        let mut i = 0usize;
        while i < wp.equipped_items.len() {
            let e = &wp.equipped_items[i];
            let idx = if let EquipmentType::Weapon = e.equipment_type { 0 }
                      else { if let EquipmentType::Armor = e.equipment_type { 1 } else { 2 } };
            w_power = w_power.saturating_add(e.power_level.saturating_mul(mul[idx]));
            i = i.saturating_add(1);
        }

        let mut l_power = 0u64;
        let mut j = 0usize;
        while j < lp.equipped_items.len() {
            let e = &lp.equipped_items[j];
            let idx = if let EquipmentType::Weapon = e.equipment_type { 0 }
                      else { if let EquipmentType::Armor = e.equipment_type { 1 } else { 2 } };
            l_power = l_power.saturating_add(e.power_level.saturating_mul(mul[idx]));
            j = j.saturating_add(1);
        }

        // スキル（固定倍率）
        w_power = w_power.saturating_add(wp.skill_level.saturating_mul(200));
        l_power = l_power.saturating_add(lp.skill_level.saturating_mul(200));

        let diff = w_power.saturating_sub(l_power);

        // 勝利種別（多段if）
        let mut vt = VictoryType::Narrow;
        if diff > 500 { vt = VictoryType::Clear; }
        if diff > 1000 { vt = VictoryType::Decisive; }

        // 報酬：単純ベース×ラウンド係数
        let base = t.prize_pool.saturating_div(16); // 固定割り当てで簡略
        let mut v = base;
        if let VictoryType::Clear = vt { v = base.saturating_mul(125) / 100; }
        if let VictoryType::Decisive = vt { v = base.saturating_mul(150) / 100; }
        let multi = t.current_round.saturating_add(1) as u64;
        let final_reward = v.saturating_mul(multi);

        // 記録
        wp.tournament_earnings = wp.tournament_earnings.saturating_add(final_reward);
        wp.wins = wp.wins.saturating_add(1);
        wp.current_win_streak = wp.current_win_streak.saturating_add(1);

        lp.losses = lp.losses.saturating_add(1);
        lp.current_win_streak = 0;

        // 連勝ボーナス（閾値のみ）
        if wp.current_win_streak >= 5 {
            let add = (wp.current_win_streak as u64).saturating_mul(100);
            wp.tournament_earnings = wp.tournament_earnings.saturating_add(add);
        }

        // ラウンド進行（残り人数は管理しない簡略版：時間で進める）
        t.current_round = t.current_round.saturating_add(1);
        t.round_start_time = now + 1800;
        t.round_end_time = t.round_start_time + 3600;

        // 決勝到達ならチャンピオン設定（簡略）
        if t.current_round >= t.max_rounds {
            t.tournament_status = TournamentStatus::Finals;
            t.champion = wp.player;
        }

        emit!(MatchCompleted {
            tournament_id: t.tournament_id,
            winner: wp.player,
            loser: lp.player,
            round: t.current_round,
            victory_type: vt.clone(),
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

#[error_code]
pub enum BracketErr {
    #[msg("Round has not started yet")] RoundNotStarted,
    #[msg("Round time has expired")] RoundExpired,
    #[msg("Tournament has been completed")] TournamentCompleted,
}

// 最小限の型（自己完結）
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

#[account]
pub struct PlayerProfile {
    pub player: Pubkey,
    pub player_name: String,
    pub skill_level: u64,
    pub wins: u32,
    pub losses: u32,
    pub current_win_streak: u32,
    pub tournament_earnings: u64,
    pub equipped_items: Vec<Equipment>,
}
#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct Equipment {
    pub item_id: u32,
    pub equipment_type: EquipmentType,
    pub power_level: u64,
}
#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub enum EquipmentType { Weapon, Armor, Accessory }
#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub enum VictoryType { Narrow, Clear, Decisive }
