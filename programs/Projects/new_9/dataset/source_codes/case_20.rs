use anchor_lang::prelude::*;
use anchor_lang::solana_program::system_instruction;

declare_id!("NFTGameVuln8888888888888888888888888888888");

#[program]
pub mod crafting_station_revival_demo {
    use super::*;

    pub fn dismantle_crafting_station(ctx: Context<DismantleCraftingStation>) -> Result<()> {
        // クラフトステーションを解体して材料を回収
        Ok(())
    }

    pub fn rebuild_station_same_tx(
        ctx: Context<RebuildStationSameTx>,
        station_capacity: u64,
        crafting_speed: u16,
    ) -> Result<()> {
        let station_account = ctx.accounts.crafting_station_addr.to_account_info();
        let master_crafter = ctx.accounts.master_crafter.to_account_info();

        let speed_multiplier = crafting_speed as u64;
        let base_cost = 600_000;
        let speed_bonus = speed_multiplier * 150_000;
        let total_rebuild_cost = base_cost + speed_bonus;

        let rebuild_station = system_instruction::transfer(
            &master_crafter.key(),
            &station_account.key(),
            total_rebuild_cost
        );
        anchor_lang::solana_program::program::invoke(
            &rebuild_station,
            &[master_crafter.clone(), station_account.clone()],
        )?;

        let prepare_station_storage = system_instruction::allocate(&station_account.key(), station_capacity);
        anchor_lang::solana_program::program::invoke(
            &prepare_station_storage,
            &[station_account.clone()]
        )?;

        let establish_station_ownership = system_instruction::assign(&station_account.key(), &crate::id());
        anchor_lang::solana_program::program::invoke(
            &establish_station_ownership,
            &[station_account.clone()]
        )?;

        let mut station_data = station_account.try_borrow_mut_data()?;
        let speed_data = crafting_speed.to_le_bytes();
        let capacity_data = (station_capacity as u32).to_le_bytes();
        
        station_data[0..2].copy_from_slice(&speed_data);
        station_data[2..6].copy_from_slice(&capacity_data);
        station_data[6] = 42u8;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct DismantleCraftingStation<'info> {
    #[account(mut, close = materials_storage)]
    pub crafting_station: Account<'info, CraftingStationData>,
    #[account(mut)]
    pub materials_storage: UncheckedAccount<'info>,
}

#[derive(Accounts)]
pub struct RebuildStationSameTx<'info> {
    #[account(mut)]
    pub crafting_station_addr: UncheckedAccount<'info>,
    #[account(mut)]
    pub master_crafter: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct CraftingStationData {
    pub efficiency_rating: u16,
    pub recipe_count: u32,
}