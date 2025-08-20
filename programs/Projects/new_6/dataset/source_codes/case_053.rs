// 07. Crafting System - Master vs Apprentice confusion
use anchor_lang::prelude::*;

declare_id!("CraftSys777777777777777777777777777777777777");

#[program]
pub mod crafting_system {
    use super::*;

    pub fn init_crafting_station(ctx: Context<InitCraftingStation>, station_type: u8, efficiency: u8) -> Result<()> {
        let station = &mut ctx.accounts.crafting_station;
        station.master_crafter = ctx.accounts.master.key();
        station.station_type = station_type;
        station.efficiency_rating = efficiency;
        station.total_crafts = 0;
        station.durability = 1000;
        station.upgrade_level = 1;
        Ok(())
    }

    pub fn execute_crafting(ctx: Context<ExecuteCrafting>, recipe_id: u32, quantity: u8) -> Result<()> {
        let station = &mut ctx.accounts.crafting_station;
        let crafter = &ctx.accounts.crafter;
        
        // Vulnerable: Any account can use the crafting station
        let base_cost = 10;
        let efficiency_bonus = station.efficiency_rating as u32;
        
        // Complex crafting calculations with loops
        for batch in 0..quantity {
            station.total_crafts += 1;
            station.durability -= 5;
            
            // Quality calculation based on multiple factors
            let quality_roll = (batch as u32 + efficiency_bonus + recipe_id) % 100;
            if quality_roll > 80 {
                station.rare_crafts += 1;
            }
        }
        
        // Resource consumption simulation
        for resource_type in 0..5 {
            station.resource_consumption[resource_type] += quantity as u32 * (resource_type + 1) as u32;
        }
        
        // Experience and leveling
        station.master_experience += quantity as u64 * 15;
        if station.master_experience >= 1000 * station.upgrade_level as u64 {
            station.upgrade_level += 1;
            station.efficiency_rating = (station.efficiency_rating + 5).min(100);
        }
        
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitCraftingStation<'info> {
    #[account(init, payer = master, space = 8 + 600)]
    pub crafting_station: Account<'info, CraftingStation>,
    #[account(mut)]
    pub master: AccountInfo<'info>, // No master crafter verification
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ExecuteCrafting<'info> {
    #[account(mut)]
    pub crafting_station: Account<'info, CraftingStation>,
    pub crafter: AccountInfo<'info>, // Could be anyone, not just master or authorized
}

#[account]
pub struct CraftingStation {
    pub master_crafter: Pubkey,
    pub station_type: u8,
    pub efficiency_rating: u8,
    pub total_crafts: u32,
    pub durability: u32,
    pub upgrade_level: u32,
    pub rare_crafts: u32,
    pub master_experience: u64,
    pub resource_consumption: [u32; 5],
}