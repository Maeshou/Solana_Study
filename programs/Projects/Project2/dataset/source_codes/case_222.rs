use anchor_lang::prelude::*;
use std::collections::BTreeMap;

declare_id!("BattlArena1111111111111111111111111111111");

#[program]
pub mod battle_arena {
    use super::*;

    pub fn fight(ctx: Context<Fight>, attack: u64, defense: u64) -> Result<()> {
        let stats = &mut ctx.accounts.stats;
        if attack > defense {
            // 勝利時
            stats.wins = stats.wins.saturating_add(1);
            stats.experience = stats.experience.saturating_add(attack - defense);
            stats.stamina = stats.stamina.saturating_sub(5);
            stats.bonus_points = stats.bonus_points.saturating_add(10);
        } else {
            // 敗北時
            stats.losses = stats.losses.saturating_add(1);
            let penalty = defense.saturating_sub(attack);
            stats.stamina = stats.stamina.saturating_sub(3);
            stats.penalty_count = stats.penalty_count.saturating_add(penalty);
            stats.recover_map.insert(stats.player, penalty);
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Fight<'info> {
    #[account(mut, seeds = [b"stats", player.key().as_ref()], bump)]
    pub stats: Account<'info, PlayerStats>,
    pub player: Signer<'info>,
}

#[account]
pub struct PlayerStats {
    pub player: Pubkey,
    pub wins: u64,
    pub losses: u64,
    pub experience: u64,
    pub stamina: u64,
    pub bonus_points: u64,
    pub penalty_count: u64,
    pub recover_map: BTreeMap<Pubkey, u64>,
}
