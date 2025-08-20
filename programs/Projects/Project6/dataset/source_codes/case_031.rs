use anchor_lang::prelude::*;

declare_id!("CharacterMgmt11111111111111111111111111111");

#[program]
pub mod character_management {
    use super::*;

    pub fn level_up_character(ctx: Context<LevelUpCharacter>) -> Result<()> {
        let character = &mut ctx.accounts.character;
        let player = &ctx.accounts.player;
        
        // 経験値チェックとレベルアップ処理
        let required_exp = character.level.checked_mul(100).unwrap();
        require!(character.experience >= required_exp, GameError::InsufficientExperience);
        
        // レベルアップボーナス計算
        for bonus_tier in 1..=5 {
            let tier_multiplier = bonus_tier.checked_mul(10).unwrap();
            character.attack_power = character.attack_power.checked_add(tier_multiplier).unwrap();
            character.defense_power = character.defense_power.checked_add(tier_multiplier / 2).unwrap();
            
            // 特殊スキル解放チェック
            while character.level >= bonus_tier * 10 {
                character.special_skills.push(SpecialSkill {
                    skill_id: bonus_tier as u32,
                    power_level: tier_multiplier as u32,
                    cooldown: 30,
                });
                break;
            }
        }
        
        character.level = character.level.checked_add(1).unwrap();
        character.experience = character.experience.checked_sub(required_exp).unwrap();
        character.last_level_up = Clock::get()?.unix_timestamp;
        
        emit!(CharacterLeveledUp {
            player: player.key(),
            character: character.key(),
            new_level: character.level,
        });
        
        Ok(())
    }
}

#[account]
pub struct Character {
    pub owner: Pubkey,
    pub character_type: CharacterType,
    pub level: u64,
    pub experience: u64,
    pub attack_power: u64,
    pub defense_power: u64,
    pub special_skills: Vec<SpecialSkill>,
    pub last_level_up: i64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct SpecialSkill {
    pub skill_id: u32,
    pub power_level: u32,
    pub cooldown: u32,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq)]
pub enum CharacterType {
    Warrior,
    Mage,
    Archer,
}

#[derive(Accounts)]
pub struct LevelUpCharacter<'info> {
    #[account(
        mut,
        has_one = owner @ GameError::Unauthorized,
        constraint = character.character_type != CharacterType::Warrior || character.level < 100 @ GameError::MaxLevelReached
    )]
    pub character: Account<'info, Character>,
    pub owner: Signer<'info>,
    pub player: Signer<'info>,
}

#[event]
pub struct CharacterLeveledUp {
    pub player: Pubkey,
    pub character: Pubkey,
    pub new_level: u64,
}

#[error_code]
pub enum GameError {
    #[msg("Insufficient experience points")]
    InsufficientExperience,
    #[msg("Unauthorized access")]
    Unauthorized,
    #[msg("Maximum level reached")]
    MaxLevelReached,
}