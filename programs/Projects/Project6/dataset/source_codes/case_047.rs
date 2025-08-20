use anchor_lang::prelude::*;

declare_id!("EVOLV222222222222222222222222222222222222222");

#[program]
pub mod evolution_program {
    use super::*;
    /// キャラクターを上位クラス「Archon」に進化させる
    pub fn evolve_character_to_archon(ctx: Context<EvolveCharacter>) -> Result<()> {
        let character = &mut ctx.accounts.player_character;
        let inventory = &mut ctx.accounts.player_inventory;
        
        inventory.items.retain(|item| item.item_id != 999); // 進化アイテム(ID: 999)を消費
        
        character.class = CharacterClass::Archon;
        character.stats.max_health = character.stats.max_health.saturating_add(500);
        character.stats.max_mana = character.stats.max_mana.saturating_add(300);
        character.stats.strength = character.stats.strength.saturating_add(50);
        character.stats.intelligence = character.stats.intelligence.saturating_add(75);
        character.stats.level = character.stats.level.saturating_add(1);
        character.stats.current_health = character.stats.max_health;
        
        msg!("Character has evolved into an Archon!");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct EvolveCharacter<'info> {
    #[account(
        mut, has_one = owner,
        constraint = player_character.stats.level >= 50 @ GameErrorCode::LevelTooLow,
        constraint = player_inventory.items.iter().any(|item| item.item_id == 999) @ GameErrorCode::MissingEvolutionItem,
    )]
    pub player_character: Account<'info, PlayerCharacter>,
    #[account(mut, has_one = owner)]
    pub player_inventory: Account<'info, PlayerInventory>,
    #[account(mut)]
    pub owner: Signer<'info>,
}

#[account]
pub struct PlayerCharacter {
    pub owner: Pubkey,
    pub class: CharacterClass,
    pub stats: CharacterStats,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
pub struct CharacterStats {
    pub level: u32,
    pub strength: u32,
    pub intelligence: u32,
    pub current_health: u64,
    pub max_health: u64,
    pub max_mana: u32,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq)]
pub enum CharacterClass { Warrior, Mage, Rogue, Archon }

#[account]
pub struct PlayerInventory {
    pub owner: Pubkey,
    pub items: Vec<Item>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
pub struct Item {
    pub item_id: u32,
    pub quantity: u32,
}

#[error_code]
pub enum GameErrorCode {
    #[msg("The character is not at a high enough level.")]
    LevelTooLow,
    #[msg("The required evolution item is not present in the inventory.")]
    MissingEvolutionItem,
}