use anchor_lang::prelude::*;

declare_id!("LEVELAAAAAAAAAAAAAABBBBBBBBBBBBBCCCCCCCCCCCD");

#[program]
pub mod leveling_program {
    use super::*;
    /// 溜まった経験値をレベルに変換する
    pub fn convert_experience_to_levels(ctx: Context<ConvertExperience>) -> Result<()> {
        let character = &mut ctx.accounts.player_character;
        
        // トランザクションごとに最大5レベルまで上昇
        for _ in 1..6 {
            let has_enough_exp = character.stats.experience >= character.experience_to_next_level;
            let level_up_flag = has_enough_exp as u64;

            character.stats.experience = character.stats.experience.saturating_sub(character.experience_to_next_level * level_up_flag);
            character.stats.level = character.stats.level.saturating_add(level_up_flag as u32);
            character.stats.strength = character.stats.strength.saturating_add(3 * level_up_flag as u32);
            
            let next_level_exp_increase = (character.experience_to_next_level / 10) * level_up_flag; // 1.1倍
            character.experience_to_next_level = character.experience_to_next_level.saturating_add(next_level_exp_increase);
        }

        msg!("Experience converted. Current level: {}", character.stats.level);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct ConvertExperience<'info> {
    #[account(mut, has_one = owner)]
    pub player_character: Account<'info, PlayerCharacter>,
    pub owner: Signer<'info>,
}

#[account]
pub struct PlayerCharacter {
    pub owner: Pubkey,
    pub stats: CharacterStats,
    pub experience_to_next_level: u64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
pub struct CharacterStats {
    pub level: u32,
    pub experience: u64,
    pub strength: u32,
}