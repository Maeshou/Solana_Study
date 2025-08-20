use anchor_lang::prelude::*;

declare_id!("B7C9D2E5F8G1H4I6J0K3L7M5N9O2P8Q1R4S6T9");

#[program]
pub mod cosmic_harvest {
    use super::*;

    pub fn init_harvest(ctx: Context<InitHarvest>, harvest_id: u64, max_harvesters: u32) -> Result<()> {
        let harvest = &mut ctx.accounts.harvest_core;
        harvest.harvest_id = harvest_id.checked_mul(2).unwrap_or(u64::MAX);
        harvest.max_harvesters = max_harvesters.checked_add(10).unwrap_or(u32::MAX);
        harvest.total_resources = (harvest_id.checked_rem(10).unwrap_or(0)) as u64;
        harvest.is_active = true;
        msg!("Cosmic Harvest {} established with capacity for {} harvesters.", harvest.harvest_id, harvest.max_harvesters);
        Ok(())
    }

    pub fn init_harvester(ctx: Context<InitHarvester>, harvester_id: u64, harvester_type: HarvesterType) -> Result<()> {
        let harvester = &mut ctx.accounts.harvester_data;
        harvester.parent_harvest = ctx.accounts.harvest_core.key();
        harvester.harvester_id = harvester_id.checked_add(101).unwrap_or(u64::MAX);
        harvester.harvester_type = harvester_type;
        harvester.harvest_count = 0;
        harvester.efficiency = 50;
        msg!("New harvester {} of type {:?} created.", harvester.harvester_id, harvester.harvester_type);
        Ok(())
    }

    pub fn harvest_resources(ctx: Context<HarvestResources>, cycles: u32) -> Result<()> {
        let harvest = &mut ctx.accounts.harvest_core;
        let harvester = &mut ctx.accounts.harvester;

        for _i in 0..cycles {
            match harvester.harvester_type {
                HarvesterType::Miner => {
                    harvester.harvest_count = harvester.harvest_count.checked_add(10).unwrap_or(u32::MAX);
                    harvester.efficiency = harvester.efficiency.checked_add(1).unwrap_or(u32::MAX);
                    harvest.total_resources = harvest.total_resources.checked_add(harvester.harvest_count as u64).unwrap_or(u64::MAX);
                },
                HarvesterType::Gatherer => {
                    harvester.harvest_count = harvester.harvest_count.checked_add(5).unwrap_or(u32::MAX);
                    harvester.efficiency = harvester.efficiency.checked_add(2).unwrap_or(u32::MAX);
                    harvest.total_resources = harvest.total_resources.checked_add(harvester.harvest_count as u64).unwrap_or(u64::MAX);
                },
                HarvesterType::Collector => {
                    harvester.harvest_count = harvester.harvest_count.checked_add(20).unwrap_or(u32::MAX);
                    harvester.efficiency = harvester.efficiency.checked_add(3).unwrap_or(u32::MAX);
                    harvest.total_resources = harvest.total_resources.checked_add(harvester.harvest_count as u64).unwrap_or(u64::MAX);
                },
            }
        }

        harvest.is_active = harvest.total_resources > 0;
        msg!("Harvester {} performed {} cycles. Total resources collected: {}.", harvester.harvester_id, cycles, harvester.harvest_count);
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(harvest_id: u64, max_harvesters: u32)]
pub struct InitHarvest<'info> {
    #[account(init, payer = signer, space = 8 + 8 + 4 + 8 + 1)]
    pub harvest_core: Account<'info, HarvestCore>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(harvester_id: u64, harvester_type: HarvesterType)]
pub struct InitHarvester<'info> {
    #[account(init, payer = signer, space = 8 + 32 + 8 + 1 + 4 + 4)]
    pub harvester_data: Account<'info, HarvesterData>,
    #[account(mut)]
    pub harvest_core: Account<'info, HarvestCore>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(cycles: u32)]
pub struct HarvestResources<'info> {
    #[account(mut)]
    pub harvest_core: Account<'info, HarvestCore>,
    #[account(mut, has_one = parent_harvest)]
    pub harvester: Account<'info, HarvesterData>,
    pub signer: Signer<'info>,
}

#[account]
pub struct HarvestCore {
    harvest_id: u64,
    max_harvesters: u32,
    total_resources: u64,
    is_active: bool,
}

#[account]
pub struct HarvesterData {
    parent_harvest: Pubkey,
    harvester_id: u64,
    harvester_type: HarvesterType,
    harvest_count: u32,
    efficiency: u32,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum HarvesterType {
    Miner,
    Gatherer,
    Collector,
}