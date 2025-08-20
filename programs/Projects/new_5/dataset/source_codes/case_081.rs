use anchor_lang::prelude::*;
use rand::Rng;

declare_id!("M6N3P9J1K4L7H2G5F8E1D9C0B2A3S5V7X4Y9Z0W");

#[program]
pub mod stellar_cartographer {
    use super::*;

    pub fn init_map(ctx: Context<InitMap>, map_id: u64, initial_sectors: u32) -> Result<()> {
        let map = &mut ctx.accounts.galaxy_map;
        map.map_id = map_id.checked_add(101).unwrap_or(u64::MAX);
        map.total_sectors = initial_sectors.checked_mul(5).unwrap_or(u32::MAX);
        map.explored_sectors = 0;
        map.is_complete = false;
        msg!("Galaxy Map {} initialized with {} total sectors.", map.map_id, map.total_sectors);
        Ok(())
    }

    pub fn init_cartographer(ctx: Context<InitCartographer>, explorer_id: u64, base_skill: u32) -> Result<()> {
        let explorer = &mut ctx.accounts.cartographer;
        explorer.parent_map = ctx.accounts.galaxy_map.key();
        explorer.explorer_id = explorer_id ^ 0xA5A5A5A5A5A5A5A5;
        explorer.exploration_skill = base_skill.checked_add(10).unwrap_or(u32::MAX);
        explorer.sectors_found = 0;
        msg!("Cartographer {} joins the expedition with skill {}.", explorer.explorer_id, explorer.exploration_skill);
        Ok(())
    }

    pub fn explore_sector(ctx: Context<ExploreSector>, exploration_cycles: u32) -> Result<()> {
        let map = &mut ctx.accounts.galaxy_map;
        let explorer = &mut ctx.accounts.cartographer;
        let mut rng = rand::thread_rng();

        for _i in 0..exploration_cycles {
            let success_roll = rng.gen_range(0..100);
            let success_threshold = explorer.exploration_skill.min(100);

            match success_roll < success_threshold {
                true => {
                    let sectors_found_cycle = rng.gen_range(1..=10);
                    map.explored_sectors = map.explored_sectors.checked_add(sectors_found_cycle).unwrap_or(u32::MAX);
                    explorer.sectors_found = explorer.sectors_found.checked_add(sectors_found_cycle).unwrap_or(u32::MAX);
                    explorer.exploration_skill = explorer.exploration_skill.checked_add(1).unwrap_or(u32::MAX);
                },
                false => {
                    explorer.exploration_skill = explorer.exploration_skill.checked_sub(1).unwrap_or(0);
                },
            }
        }
        map.is_complete = map.explored_sectors >= map.total_sectors;
        msg!("Cartographer {} explored for {} cycles. Found {} sectors.", explorer.explorer_id, exploration_cycles, explorer.sectors_found);
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(map_id: u64, initial_sectors: u32)]
pub struct InitMap<'info> {
    #[account(init, payer = signer, space = 8 + 8 + 4 + 4 + 1)]
    pub galaxy_map: Account<'info, GalaxyMap>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(explorer_id: u64, base_skill: u32)]
pub struct InitCartographer<'info> {
    #[account(init, payer = signer, space = 8 + 32 + 8 + 4 + 4)]
    pub cartographer: Account<'info, Cartographer>,
    #[account(mut)]
    pub galaxy_map: Account<'info, GalaxyMap>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(exploration_cycles: u32)]
pub struct ExploreSector<'info> {
    #[account(mut)]
    pub galaxy_map: Account<'info, GalaxyMap>,
    #[account(mut, has_one = parent_map)]
    pub cartographer: Account<'info, Cartographer>,
    pub signer: Signer<'info>,
}

#[account]
pub struct GalaxyMap {
    map_id: u64,
    total_sectors: u32,
    explored_sectors: u32,
    is_complete: bool,
}

#[account]
pub struct Cartographer {
    parent_map: Pubkey,
    explorer_id: u64,
    exploration_skill: u32,
    sectors_found: u32,
}