use anchor_lang::prelude::*;

declare_id!("RAIDB444444444444444444444444444444444444444");

#[program]
pub mod raid_program {
    use super::*;
    /// 参加者全員でレイドボスを攻撃する
    pub fn execute_raid_attack(ctx: Context<ExecuteRaidAttack>) -> Result<()> {
        let raid_boss = &mut ctx.accounts.raid_boss;
        let participants = &ctx.remaining_accounts;

        let mut total_damage: u64 = 1;

        for participant_account in participants.iter() {
            let mut participant = Account::<PlayerCharacter>::try_from(participant_account)?;
            let damage = participant.stats.strength.saturating_add(participant.stats.dexterity / 2);
            participant.stats.experience = participant.stats.experience.saturating_add(50);
            total_damage = total_damage.saturating_add(damage as u64);
        }

        raid_boss.current_health = raid_boss.current_health.saturating_sub(total_damage);
        
        msg!("Raid attack executed. Total damage: {}.", total_damage);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct ExecuteRaidAttack<'info> {
    #[account(mut)]
    pub raid_boss: Account<'info, RaidBoss>,
    pub signer: Signer<'info>, // 攻撃の実行者
}

#[account]
pub struct RaidBoss {
    pub current_health: u64,
}

#[account]
pub struct PlayerCharacter {
    pub stats: CharacterStats,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
pub struct CharacterStats {
    pub experience: u64,
    pub strength: u32,
    pub dexterity: u32,
}