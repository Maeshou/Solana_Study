use anchor_lang::prelude::*;

declare_id!("ItemCrafting1111111111111111111111111111111");

#[program]
pub mod item_crafting {
    use super::*;

    pub fn craft_legendary_weapon(ctx: Context<CraftWeapon>) -> Result<()> {
        let crafting_station = &mut ctx.accounts.crafting_station;
        let player_inventory = &mut ctx.accounts.player_inventory;
        
        // 必要材料の検証と消費
        let mut material_cost_total = 0u64;
        let required_materials = vec![
            (MaterialType::DragonScale, 5),
            (MaterialType::MithrilOre, 10),
            (MaterialType::PhoenixFeather, 2),
        ];
        
        for (material_type, required_amount) in required_materials.iter() {
            let available_amount = player_inventory.materials.get(material_type).unwrap_or(&0);
            require!(*available_amount >= *required_amount, CraftingError::InsufficientMaterials);
            
            // 材料コスト計算
            let base_cost = match material_type {
                MaterialType::DragonScale => 1000,
                MaterialType::MithrilOre => 500,
                MaterialType::PhoenixFeather => 2000,
            };
            material_cost_total = material_cost_total.checked_add(
                base_cost.checked_mul(*required_amount as u64).unwrap()
            ).unwrap();
            
            // 材料消費処理
            player_inventory.materials.insert(*material_type, 
                available_amount.checked_sub(*required_amount).unwrap());
        }
        
        // クラフト成功率計算
        let mut success_rate = 70u8;
        let crafting_level_bonus = crafting_station.crafting_level / 5;
        success_rate = success_rate.saturating_add(crafting_level_bonus as u8);
        
        // ランダム要素によるクラフト判定
        let clock = Clock::get()?;
        let random_seed = clock.unix_timestamp as u64;
        let craft_success = (random_seed % 100) < success_rate as u64;
        
        // 失敗時の処理
        if !craft_success {
            // 部分的な材料返還
            for (material_type, required_amount) in required_materials.iter() {
                let return_amount = required_amount / 2;
                let current_amount = player_inventory.materials.get(material_type).unwrap_or(&0);
                player_inventory.materials.insert(*material_type,
                    current_amount.checked_add(return_amount).unwrap());
            }
            return Err(CraftingError::CraftingFailed.into());
        }
        
        // 成功時の武器生成
        let weapon_stats = WeaponStats {
            attack_damage: 250 + (crafting_station.crafting_level * 5),
            critical_rate: 15 + (crafting_station.crafting_level / 2),
            durability: 1000,
            enchantment_slots: 3,
        };
        
        player_inventory.weapons.push(CraftedWeapon {
            weapon_id: player_inventory.weapons.len() as u32,
            weapon_type: WeaponType::LegendarySword,
            stats: weapon_stats,
            crafted_at: clock.unix_timestamp,
            crafter: ctx.accounts.player.key(),
        });
        
        crafting_station.crafting_level = crafting_station.crafting_level.checked_add(1).unwrap();
        crafting_station.total_items_crafted = crafting_station.total_items_crafted.checked_add(1).unwrap();
        
        emit!(WeaponCrafted {
            player: ctx.accounts.player.key(),
            weapon_type: WeaponType::LegendarySword,
            success: true,
        });
        
        Ok(())
    }
}

#[account]
pub struct CraftingStation {
    pub owner: Pubkey,
    pub crafting_level: u64,
    pub total_items_crafted: u64,
    pub station_type: StationType,
}

#[account]
pub struct PlayerInventory {
    pub owner: Pubkey,
    pub materials: std::collections::HashMap<MaterialType, u64>,
    pub weapons: Vec<CraftedWeapon>,
    pub max_capacity: u32,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct CraftedWeapon {
    pub weapon_id: u32,
    pub weapon_type: WeaponType,
    pub stats: WeaponStats,
    pub crafted_at: i64,
    pub crafter: Pubkey,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct WeaponStats {
    pub attack_damage: u64,
    pub critical_rate: u8,
    pub durability: u32,
    pub enchantment_slots: u8,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq, Hash)]
pub enum MaterialType {
    DragonScale,
    MithrilOre,
    PhoenixFeather,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq)]
pub enum WeaponType {
    LegendarySword,
    MysticBow,
    ElementalStaff,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq)]
pub enum StationType {
    BasicForge,
    MasterWorkshop,
    LegendaryAnvil,
}

#[derive(Accounts)]
pub struct CraftWeapon<'info> {
    #[account(
        mut,
        has_one = owner @ CraftingError::Unauthorized,
        constraint = crafting_station.station_type != StationType::BasicForge @ CraftingError::InsufficientStationLevel
    )]
    pub crafting_station: Account<'info, CraftingStation>,
    #[account(
        mut,
        has_one = owner @ CraftingError::Unauthorized
    )]
    pub player_inventory: Account<'info, PlayerInventory>,
    pub owner: Signer<'info>,
    pub player: Signer<'info>,
}

#[event]
pub struct WeaponCrafted {
    pub player: Pubkey,
    pub weapon_type: WeaponType,
    pub success: bool,
}

#[error_code]
pub enum CraftingError {
    #[msg("Insufficient materials")]
    InsufficientMaterials,
    #[msg("Crafting failed")]
    CraftingFailed,
    #[msg("Insufficient station level")]
    InsufficientStationLevel,
    #[msg("Unauthorized access")]
    Unauthorized,
}