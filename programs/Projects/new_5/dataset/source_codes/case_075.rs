// 3. プログラム名: CyberCitadels
use anchor_lang::prelude::*;

declare_id!("T5K2W8C1P4L9M7J6V3R0S5X2Y1Z0B7A9C5D4E");

#[program]
pub mod cyber_citadels {
    use super::*;

    pub fn init_citadel(ctx: Context<InitCitadel>, core_id: u64, defense_level: u32) -> Result<()> {
        let citadel = &mut ctx.accounts.citadel_core;
        citadel.core_id = core_id.checked_add(100000).unwrap_or(u64::MAX);
        citadel.defense_level = defense_level.checked_sub(100).unwrap_or(0);
        citadel.resource_reserve = core_id.checked_mul(100).unwrap_or(1);
        citadel.citadel_state = CitadelState::Peaceful;
        msg!("Citadel core {} established with defense level {}.", citadel.core_id, citadel.defense_level);
        Ok(())
    }

    pub fn init_module(ctx: Context<InitModule>, module_id: u32, module_type: u8) -> Result<()> {
        let module = &mut ctx.accounts.defense_module;
        module.parent_citadel = ctx.accounts.citadel_core.key();
        module.module_id = module_id.checked_add(50).unwrap_or(u32::MAX);
        module.module_type = module_type.rotate_right(3);
        module.is_online = (module_id % 2) != 0;
        module.energy_consumption = module_id.checked_rem(100).unwrap_or(1);
        module.last_update_cycle = module_id.checked_rem(50).unwrap_or(0);
        msg!("Module {} installed with type {}.", module.module_id, module.module_type);
        Ok(())
    }

    pub fn activate_modules(ctx: Context<ActivateModules>, energy_injection: u32) -> Result<()> {
        let citadel = &mut ctx.accounts.citadel_core;
        let module_primary = &mut ctx.accounts.module_primary;
        let module_secondary = &mut ctx.accounts.module_secondary;
        let mut loop_counter = 0;

        while loop_counter < 10 {
            if module_primary.is_online {
                let energy_use = energy_injection.checked_add(loop_counter).unwrap_or(u32::MAX);
                citadel.resource_reserve = citadel.resource_reserve.checked_sub(energy_use as u64).unwrap_or(0);
                module_primary.energy_consumption = module_primary.energy_consumption.checked_add(energy_use).unwrap_or(u32::MAX);
                citadel.defense_level = citadel.defense_level.checked_add(energy_use).unwrap_or(u32::MAX);
                module_primary.last_update_cycle = loop_counter;
                msg!("Primary module consumed {} energy, defense level increased.", energy_use);
            } else {
                let repair_cost = 1000u64.checked_div(citadel.defense_level as u64).unwrap_or(10);
                module_primary.is_online = (module_primary.module_type % 2) == 0;
                citadel.resource_reserve = citadel.resource_reserve.checked_sub(repair_cost).unwrap_or(0);
                module_primary.energy_consumption = module_primary.energy_consumption.checked_div(2).unwrap_or(0);
                module_primary.last_update_cycle = module_primary.last_update_cycle.checked_add(10).unwrap_or(u32::MAX);
                msg!("Primary module is brought back online at a cost of {}.", repair_cost);
            }

            if module_secondary.is_online {
                let energy_use = energy_injection.checked_mul(loop_counter.checked_add(1).unwrap_or(1)).unwrap_or(u32::MAX);
                citadel.resource_reserve = citadel.resource_reserve.checked_sub(energy_use as u64).unwrap_or(0);
                module_secondary.energy_consumption = module_secondary.energy_consumption.checked_add(energy_use).unwrap_or(u32::MAX);
                citadel.defense_level = citadel.defense_level.checked_add(energy_use).unwrap_or(u32::MAX);
                module_secondary.last_update_cycle = loop_counter;
                msg!("Secondary module consumed {} energy, defense level increased.", energy_use);
            } else {
                let repair_cost = 2000u64.checked_div(citadel.defense_level as u64).unwrap_or(10);
                module_secondary.is_online = (module_secondary.module_type % 3) == 0;
                citadel.resource_reserve = citadel.resource_reserve.checked_sub(repair_cost).unwrap_or(0);
                module_secondary.energy_consumption = module_secondary.energy_consumption.checked_div(3).unwrap_or(0);
                module_secondary.last_update_cycle = module_secondary.last_update_cycle.checked_add(20).unwrap_or(u32::MAX);
                msg!("Secondary module is brought back online at a cost of {}.", repair_cost);
            }
            loop_counter = loop_counter.checked_add(1).unwrap_or(u32::MAX);
        }
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(core_id: u64, defense_level: u32)]
pub struct InitCitadel<'info> {
    #[account(init, payer = signer, space = 8 + 8 + 4 + 8 + 4)]
    pub citadel_core: Account<'info, CitadelCore>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(module_id: u32, module_type: u8)]
pub struct InitModule<'info> {
    #[account(init, payer = signer, space = 8 + 32 + 4 + 1 + 1 + 4 + 4)]
    pub defense_module: Account<'info, DefenseModule>,
    #[account(mut)]
    pub citadel_core: Account<'info, CitadelCore>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(energy_injection: u32)]
pub struct ActivateModules<'info> {
    #[account(mut)]
    pub citadel_core: Account<'info, CitadelCore>,
    #[account(mut, has_one = parent_citadel)]
    pub module_primary: Account<'info, DefenseModule>,
    #[account(mut, has_one = parent_citadel)]
    pub module_secondary: Account<'info, DefenseModule>,
    pub signer: Signer<'info>,
}

#[account]
pub struct CitadelCore {
    core_id: u64,
    defense_level: u32,
    resource_reserve: u64,
    citadel_state: CitadelState,
}

#[account]
pub struct DefenseModule {
    parent_citadel: Pubkey,
    module_id: u32,
    module_type: u8,
    is_online: bool,
    energy_consumption: u32,
    last_update_cycle: u32,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum CitadelState {
    Peaceful,
    UnderAttack,
    Offline,
}
