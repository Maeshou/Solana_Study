use anchor_lang::prelude::*;

declare_id!("COMBT666666666666666666666666666666666666666");

#[program]
pub mod combat_ability_program {
    use super::*;
    /// 特殊アビリティ「メテオ」を使用する
    pub fn cast_meteor_spell(ctx: Context<CastAbility>, damage_multiplier: u32) -> Result<()> {
        let character = &mut ctx.accounts.player_character;
        let monster = &mut ctx.accounts.monster;
        let clock = Clock::get()?;

        character.stats.current_mana = character.stats.current_mana.saturating_sub(150);
        
        let base_damage = character.stats.intelligence * damage_multiplier;
        let final_damage = base_damage.saturating_sub(monster.magic_resistance);
        monster.current_health = monster.current_health.saturating_sub(final_damage as u64);

        character.last_ability_timestamp = clock.unix_timestamp + 300; // 5分間のクールダウン

        msg!("Meteor cast! Dealt {} damage.", final_damage);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CastAbility<'info> {
    #[account(
        mut, has_one = owner,
        constraint = player_character.stats.current_mana >= 150 @ GameErrorCode::NotEnoughEnergy,
        constraint = Clock::get().unwrap().unix_timestamp > player_character.last_ability_timestamp @ GameErrorCode::CharacterOnCooldown
    )]
    pub player_character: Account<'info, PlayerCharacter>,
    #[account(mut)]
    pub monster: Account<'info, Monster>,
    pub owner: Signer<'info>,
}

#[account]
pub struct PlayerCharacter {
    pub owner: Pubkey,
    pub stats: CharacterStats,
    pub last_ability_timestamp: i64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
pub struct CharacterStats {
    pub intelligence: u32,
    pub current_mana: u32,
}

#[account]
pub struct Monster {
    pub current_health: u64,
    pub magic_resistance: u32,
}

#[error_code]
pub enum GameErrorCode {
    #[msg("Not enough energy to perform this action.")]
    NotEnoughEnergy,
    #[msg("This character is currently on a cooldown period.")]
    CharacterOnCooldown,
}