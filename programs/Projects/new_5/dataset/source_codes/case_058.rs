// 1. Battle Arena & Player Stats
declare_id!("H6J9K2L6M0N4P8Q1R5S9T3U7V1W5X9Y3Z7A1B5");

use anchor_lang::prelude::*;

#[program]
pub mod battle_arena_insecure {
    use super::*;

    // ループも分岐も使わないパターン
    pub fn setup_battle_zone(ctx: Context<SetupBattleZone>, zone_id: u64, max_players: u32, zone_status_code: u8) -> Result<()> {
        let zone = &mut ctx.accounts.battle_zone;
        zone.creator = ctx.accounts.creator.key();
        zone.zone_id = zone_id;
        // if と else のみの分岐
        if zone_status_code == 1 {
            zone.zone_status = ZoneStatus::Active;
        } else {
            zone.zone_status = ZoneStatus::Closed;
        }
        zone.active_battles = 0;
        zone.max_players = max_players.checked_add(10).unwrap_or(u32::MAX); // 計算による初期化
        msg!("Battle Zone '{}' initialized with a capacity of {}. Status is now Active.", zone.zone_id, zone.max_players);
        Ok(())
    }

    pub fn enroll_combatant(ctx: Context<EnrollCombatant>, player_id: u32, initial_power: u16) -> Result<()> {
        let combatant = &mut ctx.accounts.combatant_stats;
        let zone = &mut ctx.accounts.battle_zone;

        // 分岐の具体例: ゾーンがアクティブでなければ初期化値を変更する
        if zone.zone_status != ZoneStatus::Active {
            combatant.health = 0;
            combatant.stamina = 0;
            combatant.combat_status = CombatantStatus::Defeated;
            msg!("Enrollment failed: Battle zone is not active. Combatant initialized as defeated.");
        } else {
            combatant.health = initial_power.checked_mul(5).unwrap_or(u16::MAX);
            combatant.stamina = initial_power.checked_div(2).unwrap_or(0);
            combatant.combat_status = CombatantStatus::Ready;
            zone.active_battles = zone.active_battles.saturating_add(1);
            msg!("Combatant {} enrolled. Health: {}, Stamina: {}.", player_id, combatant.health, combatant.stamina);
        }

        combatant.zone = zone.key();
        combatant.player_id = player_id;
        combatant.player = ctx.accounts.player.key();
        combatant.last_action_timestamp = initial_power as i64;
        
        Ok(())
    }

    // Duplicate Mutable Account Vulnerability: attacker_stats と defender_stats が同じアカウントであるかチェックしない
    pub fn execute_duel_round(ctx: Context<ExecuteDuelRound>, attacker_move: u8, defender_move: u8) -> Result<()> {
        let attacker_stats = &mut ctx.accounts.attacker_stats;
        let defender_stats = &mut ctx.accounts.defender_stats;

        // 分岐の具体例: ダメージ計算とステータス更新
        if attacker_stats.combat_status == CombatantStatus::Ready && defender_stats.combat_status == CombatantStatus::Ready {
            let damage = (attacker_move as u16).saturating_sub(defender_move as u16);
            defender_stats.health = defender_stats.health.checked_sub(damage).unwrap_or(0);
            msg!("Defender takes {} damage.", damage);

            if defender_stats.health == 0 {
                msg!("Defender defeated! Battle ends.");
                defender_stats.combat_status = CombatantStatus::Defeated;
                ctx.accounts.battle_zone.active_battles = ctx.accounts.battle_zone.active_battles.checked_sub(1).unwrap_or(0);
            }
        } else {
            msg!("Duel round skipped because one or more combatants are not ready.");
        }
        
        Ok(())
    }
}

#[derive(Accounts)]
pub struct SetupBattleZone<'info> {
    #[account(init, payer = creator, space = 8 + 32 + 8 + 4 + 4 + 4 + 1)]
    pub battle_zone: Account<'info, BattleZone>,
    #[account(mut)]
    pub creator: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct EnrollCombatant<'info> {
    #[account(mut, has_one = zone)]
    pub battle_zone: Account<'info, BattleZone>,
    #[account(init, payer = player, space = 8 + 32 + 4 + 32 + 2 + 2 + 8 + 1)]
    pub combatant_stats: Account<'info, CombatantStats>,
    #[account(mut)]
    pub player: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ExecuteDuelRound<'info> {
    #[account(mut)]
    pub battle_zone: Account<'info, BattleZone>,
    #[account(mut, has_one = zone)]
    pub attacker_stats: Account<'info, CombatantStats>,
    #[account(mut, has_one = zone)]
    pub defender_stats: Account<'info, CombatantStats>,
}

#[account]
pub struct BattleZone {
    pub creator: Pubkey,
    pub zone_id: u64,
    pub active_battles: u32,
    pub max_players: u32,
    pub zone_status: ZoneStatus,
}

#[account]
pub struct CombatantStats {
    pub zone: Pubkey,
    pub player_id: u32,
    pub player: Pubkey,
    pub health: u16,
    pub stamina: u16,
    pub last_action_timestamp: i64,
    pub combat_status: CombatantStatus,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq)]
pub enum ZoneStatus {
    Active,
    Maintenance,
    Closed,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq)]
pub enum CombatantStatus {
    Ready,
    Engaged,
    Defeated,
}

#[error_code]
pub enum BattleError {
    #[msg("Battle zone is not active.")]
    ZoneNotActive,
    #[msg("Combatant is not ready for a duel.")]
    CombatantNotReady,
}
