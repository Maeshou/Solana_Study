// ==================== 1. 脆弱なNFTクラフティング ====================
// 素材アカウントと装備アカウントの検証が不十分で、同一アカウント流用による不正が可能

use anchor_lang::prelude::*;

declare_id!("V1U2L3N4E5R6A7B8L9E0C1R2A3F4T5I6N7G8N9F0");

#[program]
pub mod vulnerable_nft_crafting {
    use super::*;
    
    pub fn init_workshop(
        ctx: Context<InitWorkshop>,
        workshop_name: String,
        crafting_fee: u64,
    ) -> Result<()> {
        let workshop = &mut ctx.accounts.workshop;
        workshop.owner = ctx.accounts.owner.key();
        workshop.workshop_name = workshop_name;
        workshop.crafting_fee = crafting_fee;
        workshop.total_crafts = 0;
        workshop.is_open = true;
        
        msg!("Workshop initialized: {}", workshop.workshop_name);
        Ok(())
    }
    
    pub fn init_material_storage(
        ctx: Context<InitMaterialStorage>,
        material_type: MaterialType,
        quantity: u32,
    ) -> Result<()> {
        let storage = &mut ctx.accounts.storage;
        storage.workshop = ctx.accounts.workshop.key();
        storage.material_type = material_type;
        storage.quantity = quantity;
        storage.quality_grade = 1;
        storage.is_available = true;
        
        msg!("Material storage created: {:?}", material_type);
        Ok(())
    }
    
    pub fn execute_crafting_process(
        ctx: Context<ExecuteCraftingProcess>,
        iterations: u8,
        base_power: u16,
    ) -> Result<()> {
        let workshop = &mut ctx.accounts.workshop;
        
        // 脆弱性: material_source と equipment_target が同一アカウントでも検証されない
        let mut round = 0u8;
        loop {
            if round >= iterations { break; }
            
            if workshop.is_open {
                // オープン工房での製作処理
                workshop.total_crafts += 1;
                let craft_bonus = (round as u32) * 50;
                workshop.crafting_fee += craft_bonus as u64;
                
                // シンプルなビット演算
                let power_mod = base_power ^ (round as u16);
                let final_power = power_mod & 0xFF;
                
                msg!("Craft round {}: power {}", round, final_power);
            } else {
                // クローズ工房での調整
                if workshop.total_crafts > 0 {
                    workshop.total_crafts -= 1;
                }
                workshop.crafting_fee = workshop.crafting_fee.saturating_sub(100);
                
                msg!("Closed workshop adjustment: round {}", round);
            }
            round += 1;
        }
        
        // 最終品質計算
        let mut quality_iter = 0u8;
        while quality_iter < 3 {
            let quality_boost = quality_iter * 10;
            workshop.crafting_fee += quality_boost as u64;
            quality_iter += 1;
        }
        
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitWorkshop<'info> {
    #[account(
        init,
        payer = owner,
        space = 8 + 32 + 64 + 8 + 8 + 1
    )]
    pub workshop: Account<'info, CraftingWorkshop>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct InitMaterialStorage<'info> {
    pub workshop: Account<'info, CraftingWorkshop>,
    #[account(
        init,
        payer = manager,
        space = 8 + 32 + 1 + 4 + 4 + 1
    )]
    pub storage: Account<'info, MaterialStorage>,
    #[account(mut)]
    pub manager: Signer<'info>,
    pub system_program: Program<'info, System>,
}

// 脆弱性: 素材と装備が同じアカウントでも検証されない
#[derive(Accounts)]
pub struct ExecuteCraftingProcess<'info> {
    #[account(mut)]
    pub workshop: Account<'info, CraftingWorkshop>,
    /// CHECK: 素材アカウントの検証が不十分
    pub material_source: AccountInfo<'info>,
    /// CHECK: 装備アカウントの検証が不十分
    pub equipment_target: AccountInfo<'info>,
    pub crafter: Signer<'info>,
}

#[account]
pub struct CraftingWorkshop {
    pub owner: Pubkey,
    pub workshop_name: String,
    pub crafting_fee: u64,
    pub total_crafts: u64,
    pub is_open: bool,
}

#[account]
pub struct MaterialStorage {
    pub workshop: Pubkey,
    pub material_type: MaterialType,
    pub quantity: u32,
    pub quality_grade: u32,
    pub is_available: bool,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq)]
pub enum MaterialType {
    Iron,
    Wood,
    Crystal,
    Leather,
}

use MaterialType::*;

#[error_code]
pub enum CraftingError {
    #[msg("Workshop closed")]
    WorkshopClosed,
    #[msg("Insufficient materials")]
    InsufficientMaterials,
}