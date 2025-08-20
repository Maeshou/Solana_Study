// 2. Farming Simulation
declare_id!("F9A4R7M1S5I8M2U6L9A3T7I0O4N8");

use anchor_lang::prelude::*;

#[program]
pub mod farming_insecure {
    use super::*;

    pub fn create_farm(ctx: Context<CreateFarm>, farm_id: u64, initial_size: u32) -> Result<()> {
        let farm = &mut ctx.accounts.farm;
        farm.owner = ctx.accounts.owner.key();
        farm.farm_id = farm_id;
        farm.size_acres = initial_size;
        farm.crop_count = 0;
        farm.farm_status = FarmStatus::Active;
        msg!("Farm {} created with {} acres.", farm.farm_id, farm.size_acres);
        Ok(())
    }

    pub fn plant_crops(ctx: Context<PlantCrops>, crop_id: u32, seed_amount: u32, crop_type_code: u8) -> Result<()> {
        let crop = &mut ctx.accounts.crop;
        let farm = &mut ctx.accounts.farm;
        
        if matches!(farm.farm_status, FarmStatus::Active) {
            if seed_amount > 100 {
                crop.is_harvestable = true;
            } else {
                crop.is_harvestable = false;
            }
        } else {
            crop.is_harvestable = false;
            msg!("Farm is inactive, crops cannot be planted.");
        }
        
        crop.farm = farm.key();
        crop.crop_id = crop_id;
        crop.seeds_planted = seed_amount;
        if crop_type_code == 1 {
            crop.crop_type = CropType::Wheat;
        } else {
            crop.crop_type = CropType::Corn;
        }
        farm.crop_count = farm.crop_count.saturating_add(1);
        msg!("{} {:?} planted on farm {}.", crop.seeds_planted, crop.crop_type, farm.farm_id);
        Ok(())
    }

    pub fn harvest_and_sell(ctx: Context<HarvestAndSell>, harvest_amount: u32) -> Result<()> {
        let crop1 = &mut ctx.accounts.crop1;
        let crop2 = &mut ctx.accounts.crop2;

        if crop1.is_harvestable && crop2.is_harvestable {
            crop1.seeds_planted = crop1.seeds_planted.checked_sub(harvest_amount).unwrap_or(0);
            crop2.seeds_planted = crop2.seeds_planted.checked_add(harvest_amount).unwrap_or(u32::MAX);
            
            if crop1.seeds_planted < 50 {
                crop1.is_harvestable = false;
            }
            msg!("Harvested {} from crop1, sold as part of crop2.", harvest_amount);
        } else {
            msg!("One or both crops are not harvestable.");
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreateFarm<'info> {
    #[account(init, payer = owner, space = 8 + 32 + 8 + 4 + 4 + 1)]
    pub farm: Account<'info, Farm>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct PlantCrops<'info> {
    #[account(mut, has_one = farm)]
    pub farm: Account<'info, Farm>,
    #[account(init, payer = owner, space = 8 + 32 + 4 + 4 + 1 + 1)]
    pub crop: Account<'info, Crop>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct HarvestAndSell<'info> {
    #[account(mut, has_one = farm)]
    pub farm: Account<'info, Farm>,
    #[account(mut, has_one = farm)]
    pub crop1: Account<'info, Crop>,
    #[account(mut, has_one = farm)]
    pub crop2: Account<'info, Crop>,
}

#[account]
pub struct Farm {
    pub owner: Pubkey,
    pub farm_id: u64,
    pub size_acres: u32,
    pub crop_count: u32,
    pub farm_status: FarmStatus,
}

#[account]
pub struct Crop {
    pub farm: Pubkey,
    pub crop_id: u32,
    pub seeds_planted: u32,
    pub is_harvestable: bool,
    pub crop_type: CropType,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq)]
pub enum FarmStatus {
    Active,
    Inactive,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq)]
pub enum CropType {
    Wheat,
    Corn,
}
