// 3. Boss Arena Recorder - Referee vs Combatant Confusion
use anchor_lang::prelude::*;

declare_id!("BossArenaConflict33333333333333333333333333333333");

#[program]
pub mod boss_arena {
    use super::*;

    pub fn submit_battle_log(ctx: Context<SubmitBattleLog>, boss_id: u64, damage: u64, time_taken: u32, victory: bool) -> Result<()> {
        let arena = &mut ctx.accounts.boss_arena;
        let referee = &ctx.accounts.referee;
        let player = &ctx.accounts.combatant;
        let log = &mut ctx.accounts.battle_log;

        // Type Cosplay: refereeとcombatantの役割が混同される構造
        // refereeが本来しか許されない操作をplayerが行えてしまう

        arena.last_boss_id = boss_id;
        arena.total_battles += 1;

        let now = Clock::get()?.unix_timestamp;
        log.player = player.key();
        log.referee = referee.key();
        log.timestamp = now;
        log.boss_id = boss_id;
        log.damage = damage;
        log.time_taken = time_taken;
        log.victory = victory;

        // 複雑な報酬スキーム
        let mut score = damage;
        if victory {
            score += 100;
        }
        if time_taken < 60 {
            score += 50;
        }
        arena.total_score += score;

        // 状態遷移ログ
        if arena.highest_damage < damage {
            arena.highest_damage = damage;
            arena.top_challenger = player.key();
            log.breaks_record = true;
        } else {
            log.breaks_record = false;
        }

        // シーズン切り替え条件
        if arena.total_battles % 100 == 0 {
            arena.season_id += 1;
            arena.total_score = 0;
            arena.season_start = now;
        }

        Ok(())
    }
}

#[derive(Accounts)]
pub struct SubmitBattleLog<'info> {
    #[account(mut)]
    pub boss_arena: Account<'info, BossArenaState>,
    pub referee: AccountInfo<'info>, // 混同可能なアカウント構造
    pub combatant: AccountInfo<'info>,
    #[account(mut)]
    pub battle_log: Account<'info, BattleLog>,
}

#[account]
pub struct BossArenaState {
    pub last_boss_id: u64,
    pub total_battles: u64,
    pub total_score: u64,
    pub highest_damage: u64,
    pub top_challenger: Pubkey,
    pub season_id: u32,
    pub season_start: i64,
}

#[account]
pub struct BattleLog {
    pub player: Pubkey,
    pub referee: Pubkey,
    pub timestamp: i64,
    pub boss_id: u64,
    pub damage: u64,
    pub time_taken: u32,
    pub victory: bool,
    pub breaks_record: bool,
}
