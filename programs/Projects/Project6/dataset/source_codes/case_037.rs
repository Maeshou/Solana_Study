// 4) rewards_bank: 報酬ユーティリティ（軽量）
use anchor_lang::prelude::*;

declare_id!("ReWaRdSbAnK111111111111111111111111111");

#[program]
pub mod rewards_bank {
    use super::*;

    pub fn grant_champion_bonus(ctx: Context<ChampionBonus>) -> Result<()> {
        let t = &mut ctx.accounts.tournament;
        let p = &mut ctx.accounts.profile;
        let b = t.prize_pool / 4;
        p.tournament_earnings = p.tournament_earnings.saturating_add(b);
        Ok(())
    }

    pub fn grant_streak_bonus(ctx: Context<StreakBonus>, threshold: u32, per_step: u64) -> Result<()> {
        let p = &mut ctx.accounts.profile;
        if p.current_win_streak >= threshold {
            let steps = p.current_win_streak.saturating_sub(threshold).saturating_add(1) as u64;
            p.tournament_earnings = p.tournament_earnings.saturating_add(steps.saturating_mul(per_step));
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct ChampionBonus<'info> {
    #[account(mut)]
    pub tournament: Account<'info, Tournament>,
    #[account(mut)]
    pub profile: Account<'info, PlayerProfile>,
}
#[derive(Accounts)]
pub struct StreakBonus<'info> {
    #[account(mut)]
    pub profile: Account<'info, PlayerProfile>,
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
