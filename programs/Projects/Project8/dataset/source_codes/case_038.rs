use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod nft_game {
    use super::*;

    // NFTキャラクターをレベルアップさせる
    pub fn level_up_character(ctx: Context<LevelUpCharacter>, new_level: u16) -> Result<()> {
        let character = &mut ctx.accounts.character_stats;
        let player = &ctx.accounts.player;

        // レベルアップに必要な経験値の閾値を計算 (例: レベル * 100)
        let required_experience = u64::from(character.level) * 100;
        
        // 経験値が足りているか検証
        require!(character.experience >= required_experience, GameError::NotEnoughExperience);

        // 新しいレベルが現在のレベルより高いか検証
        require!(new_level > character.level, GameError::InvalidLevel);

        // レベルアップ処理
        character.level = new_level;
        character.attack_power += 5 * u32::from(new_level - character.level);
        character.defense_power += 3 * u32::from(new_level - character.level);
        character.experience -= required_experience; // 経験値を消費

        msg!("Character {} leveled up to {}!", character.key(), new_level);
        msg!("New Attack: {}, New Defense: {}", character.attack_power, character.defense_power);

        // 特定のレベルに到達した場合、特別なアビリティを付与するループ
        for i in (character.level - (new_level - character.level))..new_level {
            if i == 10 {
                character.special_abilities.push(1); // アビリティID: 1 を追加
                msg!("Learned new ability at level 10!");
            }
            if i == 20 {
                character.special_abilities.push(2); // アビリティID: 2 を追加
                msg!("Learned new ability at level 20!");
            }
        }

        Ok(())
    }
}

#[derive(Accounts)]
pub struct LevelUpCharacter<'info> {
    #[account(mut, 
        seeds = [b"character_stats", player.key().as_ref()], 
        bump = character_stats.bump
    )]
    pub character_stats: Account<'info, CharacterStats>,
    #[account(mut)]
    pub player: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct CharacterStats {
    pub level: u16,
    pub experience: u64,
    pub attack_power: u32,
    pub defense_power: u32,
    pub special_abilities: Vec<u8>,
    pub bump: u8,
}

#[error_code]
pub enum GameError {
    #[msg("Not enough experience to level up.")]
    NotEnoughExperience,
    #[msg("Invalid level provided.")]
    InvalidLevel,
}