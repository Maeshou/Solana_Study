// 03. Raid Record System - Leader vs Participant confusion
use anchor_lang::prelude::*;

declare_id!("RaidRec3333333333333333333333333333333333333");

#[program]
pub mod raid_recorder {
    use super::*;

    pub fn init_raid_session(ctx: Context<InitRaidSession>, raid_type: u8, difficulty: u8) -> Result<()> {
        let raid = &mut ctx.accounts.raid_session;
        raid.leader = ctx.accounts.leader.key();
        raid.raid_type = raid_type;
        raid.difficulty_level = difficulty;
        raid.participant_count = 1;
        raid.total_damage_dealt = 0;
        raid.boss_health = 10000 * (difficulty as u64);
        raid.is_completed = false;
        Ok(())
    }

    pub fn record_damage(ctx: Context<RecordDamage>, damage_amount: u32, spell_used: u8) -> Result<()> {
        let raid = &mut ctx.accounts.raid_session;
        let participant = &ctx.accounts.participant;
        
        // Vulnerable: Any account can record damage and modify raid state
        raid.total_damage_dealt += damage_amount as u64;
        
        if raid.boss_health > damage_amount as u64 {
            raid.boss_health -= damage_amount as u64;
        } else {
            raid.boss_health = 0;
            raid.is_completed = true;
        }
        
        // Multiple updates without proper authorization
        for i in 0..raid.participant_count {
            if i == 0 { // Simulate damage distribution
                raid.damage_contributions[i as usize] += damage_amount / 2;
            }
        }
        
        raid.spell_usage_count[spell_used as usize] += 1;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitRaidSession<'info> {
    #[account(init, payer = leader, space = 8 + 800)]
    pub raid_session: Account<'info, RaidData>,
    #[account(mut)]
    pub leader: AccountInfo<'info>, // No leadership verification
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct RecordDamage<'info> {
    #[account(mut)]
    pub raid_session: Account<'info, RaidData>,
    pub participant: AccountInfo<'info>, // Could be non-participant
}

#[account]
pub struct RaidData {
    pub leader: Pubkey,
    pub raid_type: u8,
    pub difficulty_level: u8,
    pub participant_count: u32,
    pub total_damage_dealt: u64,
    pub boss_health: u64,
    pub is_completed: bool,
    pub damage_contributions: [u32; 20],
    pub spell_usage_count: [u32; 10],
}
