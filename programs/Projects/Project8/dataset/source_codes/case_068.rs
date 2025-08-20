use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount, Mint};

declare_id!("11111111111111111111111111111111");

#[program]
pub mod nft_character_creation {
    use super::*;

    pub fn create_character_with_stats(
        ctx: Context<CreateCharacterStats>,
        character_name: String,
        base_strength: u32,
        base_agility: u32,
        base_intelligence: u32,
    ) -> Result<()> {
        let character_data = &mut ctx.accounts.character_account;
        let timestamp = Clock::get()?.unix_timestamp;
        
        character_data.owner = ctx.accounts.user.key();
        character_data.name = character_name;
        character_data.creation_timestamp = timestamp;
        
        let total_stats = base_strength + base_agility + base_intelligence;
        require!(total_stats <= 100, CustomError::StatsTooHigh);
        
        character_data.strength = base_strength;
        character_data.agility = base_agility;
        character_data.intelligence = base_intelligence;
        
        let experience_multiplier = match total_stats {
            score if score >= 80 => {
                character_data.rarity = CharacterRarity::Legendary;
                character_data.base_experience_rate = 150;
                character_data.special_abilities = vec![
                    "Critical Strike".to_string(),
                    "Magic Amplification".to_string(),
                    "Speed Boost".to_string()
                ];
                200
            },
            score if score >= 60 => {
                character_data.rarity = CharacterRarity::Epic;
                character_data.base_experience_rate = 125;
                character_data.special_abilities = vec![
                    "Power Attack".to_string(),
                    "Quick Recovery".to_string()
                ];
                150
            },
            _ => {
                character_data.rarity = CharacterRarity::Common;
                character_data.base_experience_rate = 100;
                character_data.special_abilities = vec!["Basic Attack".to_string()];
                100
            }
        };
        
        character_data.max_health_points = (base_strength * 10) + (base_agility * 5) + (base_intelligence * 3);
        character_data.current_health_points = character_data.max_health_points;
        character_data.mana_points = base_intelligence * 15;
        character_data.experience_points = 0;
        character_data.level = 1;
        
        let mut equipment_slots = Vec::new();
        for slot_index in 0..6 {
            equipment_slots.push(EquipmentSlot {
                slot_type: match slot_index {
                    0 => SlotType::Weapon,
                    1 => SlotType::Armor,
                    2 => SlotType::Helmet,
                    3 => SlotType::Boots,
                    4 => SlotType::Accessory,
                    _ => SlotType::Ring,
                },
                equipped_item: None,
                enhancement_level: 0,
            });
        }
        character_data.equipment = equipment_slots;
        
        character_data.combat_record = CombatRecord {
            battles_won: 0,
            battles_lost: 0,
            total_damage_dealt: 0,
            total_damage_received: 0,
            highest_critical_damage: 0,
        };
        
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreateCharacterStats<'info> {
    #[account(
        init,
        payer = user,
        space = 8 + CharacterAccount::LEN,
        seeds = [b"character", user.key().as_ref()],
        bump
    )]
    pub character_account: Account<'info, CharacterAccount>,
    
    #[account(mut)]
    pub user: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}

#[account]
pub struct CharacterAccount {
    pub owner: Pubkey,
    pub name: String,
    pub creation_timestamp: i64,
    pub strength: u32,
    pub agility: u32,
    pub intelligence: u32,
    pub rarity: CharacterRarity,
    pub base_experience_rate: u32,
    pub special_abilities: Vec<String>,
    pub max_health_points: u32,
    pub current_health_points: u32,
    pub mana_points: u32,
    pub experience_points: u64,
    pub level: u32,
    pub equipment: Vec<EquipmentSlot>,
    pub combat_record: CombatRecord,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq)]
pub enum CharacterRarity {
    Common,
    Epic,
    Legendary,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct EquipmentSlot {
    pub slot_type: SlotType,
    pub equipped_item: Option<Pubkey>,
    pub enhancement_level: u8,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub enum SlotType {
    Weapon,
    Armor,
    Helmet,
    Boots,
    Accessory,
    Ring,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct CombatRecord {
    pub battles_won: u32,
    pub battles_lost: u32,
    pub total_damage_dealt: u64,
    pub total_damage_received: u64,
    pub highest_critical_damage: u32,
}

impl CharacterAccount {
    pub const LEN: usize = 32 + 4 + 32 + 8 + 4 + 4 + 4 + 1 + 4 + 4 + 200 + 4 + 4 + 4 + 8 + 4 + 4 + 300 + 32;
}

#[error_code]
pub enum CustomError {
    #[msg("Total stats cannot exceed 100")]
    StatsTooHigh,
}