use anchor_lang::prelude::*;

declare_id!("Q5T8P2H4R7L9N6K1M3W2Y5X0B9A7C6D5E4F3G");

const MAX_ENERGY_CAPACITY: u64 = 5000;
const ENERGY_GAIN_PER_CYCLE: u64 = 100;
const MIN_PROCESSING_EFFICIENCY: u32 = 50;

#[program]
pub mod quantum_harvester {
    use super::*;

    pub fn init_harvester(ctx: Context<InitHarvester>, harvester_id: u64, energy_level: u64) -> Result<()> {
        let harvester = &mut ctx.accounts.harvester_core;
        harvester.harvester_id = harvester_id * 3;
        harvester.energy_level = energy_level;
        harvester.crystals_processed = 0;
        harvester.is_active = harvester.energy_level > 0;
        msg!("Quantum Harvester {} initialized with {} energy.", harvester.harvester_id, harvester.energy_level);
        Ok(())
    }

    pub fn init_crystal(ctx: Context<InitCrystal>, crystal_id: u64, efficiency: u32) -> Result<()> {
        let crystal = &mut ctx.accounts.quantum_crystal;
        crystal.parent_harvester = ctx.accounts.harvester_core.key();
        crystal.crystal_id = crystal_id ^ 0x1122334455667788;
        crystal.efficiency = efficiency;
        crystal.is_charged = false;
        msg!("New crystal {} created with {} efficiency.", crystal.crystal_id, crystal.efficiency);
        Ok(())
    }

    pub fn process_quantum_state(ctx: Context<ProcessQuantumState>, cycles: u32) -> Result<()> {
        let harvester = &mut ctx.accounts.harvester_core;
        let crystal_a = &mut ctx.accounts.crystal_a;
        let crystal_b = &mut ctx.accounts.crystal_b;

        for _i in 0..cycles {
            // crystal_aの処理
            if crystal_a.efficiency > MIN_PROCESSING_EFFICIENCY {
                harvester.energy_level = harvester.energy_level.saturating_add(ENERGY_GAIN_PER_CYCLE);
                crystal_a.is_charged = harvester.energy_level > MAX_ENERGY_CAPACITY / 2;
            } else {
                harvester.energy_level = harvester.energy_level.saturating_sub(ENERGY_GAIN_PER_CYCLE / 2);
            }

            // crystal_bの処理
            if crystal_b.efficiency > MIN_PROCESSING_EFFICIENCY {
                harvester.energy_level = harvester.energy_level.saturating_add(ENERGY_GAIN_PER_CYCLE * 2);
                crystal_b.is_charged = harvester.energy_level > MAX_ENERGY_CAPACITY;
            } else {
                harvester.energy_level = harvester.energy_level.saturating_sub(ENERGY_GAIN_PER_CYCLE);
            }
        }
        harvester.crystals_processed = harvester.crystals_processed.saturating_add(2);
        harvester.is_active = harvester.energy_level > 0;
        
        msg!("Processed crystals for {} cycles. Harvester energy: {}. Crystal A charged: {}, Crystal B charged: {}.", 
            cycles, harvester.energy_level, crystal_a.is_charged, crystal_b.is_charged);
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(harvester_id: u64, energy_level: u64)]
pub struct InitHarvester<'info> {
    #[account(init, payer = signer, space = 8 + 8 + 8 + 8 + 1)]
    pub harvester_core: Account<'info, HarvesterCore>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(crystal_id: u64, efficiency: u32)]
pub struct InitCrystal<'info> {
    #[account(init, payer = signer, space = 8 + 32 + 8 + 4 + 1)]
    pub quantum_crystal: Account<'info, QuantumCrystal>,
    #[account(mut)]
    pub harvester_core: Account<'info, HarvesterCore>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(cycles: u32)]
pub struct ProcessQuantumState<'info> {
    #[account(mut)]
    pub harvester_core: Account<'info, HarvesterCore>,
    #[account(mut, has_one = parent_harvester)]
    pub crystal_a: Account<'info, QuantumCrystal>,
    #[account(mut, has_one = parent_harvester)]
    pub crystal_b: Account<'info, QuantumCrystal>,
    pub signer: Signer<'info>,
}

#[account]
pub struct HarvesterCore {
    harvester_id: u64,
    energy_level: u64,
    crystals_processed: u64,
    is_active: bool,
}

#[account]
pub struct QuantumCrystal {
    parent_harvester: Pubkey,
    crystal_id: u64,
    efficiency: u32,
    is_charged: bool,
}