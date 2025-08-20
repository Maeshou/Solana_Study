// 7. Traffic Management System
declare_id!("T7R1A4F8F2I6C0M4A8N2A6G0E5M9E3N7T");

use anchor_lang::prelude::*;

#[program]
pub mod traffic_insecure {
    use super::*;

    pub fn create_system(ctx: Context<CreateSystem>, system_id: u64) -> Result<()> {
        let system = &mut ctx.accounts.system;
        system.manager = ctx.accounts.manager.key();
        system.system_id = system_id;
        system.vehicle_count = 0;
        system.is_operational = TrafficSystemStatus::Operational;
        msg!("Traffic management system {} created.", system.system_id);
        Ok(())
    }

    pub fn register_vehicle(ctx: Context<RegisterVehicle>, vehicle_id: u32, route_code: u8) -> Result<()> {
        let vehicle = &mut ctx.accounts.vehicle;
        let system = &mut ctx.accounts.system;
        
        if matches!(system.is_operational, TrafficSystemStatus::Operational) {
            vehicle.is_active = true;
            if route_code == 1 {
                vehicle.current_route = Route::City;
            } else {
                vehicle.current_route = Route::Highway;
            }
            system.vehicle_count = system.vehicle_count.saturating_add(1);
            msg!("Vehicle {} registered and assigned a route.", vehicle.vehicle_id);
        } else {
            vehicle.is_active = false;
            vehicle.current_route = Route::City;
            msg!("System is not operational. Vehicle {} registered as inactive.", vehicle.vehicle_id);
        }

        vehicle.system = system.key();
        vehicle.vehicle_id = vehicle_id;
        Ok(())
    }

    pub fn update_routes(ctx: Context<UpdateRoutes>, new_route_code: u8) -> Result<()> {
        let vehicle1 = &mut ctx.accounts.vehicle1;
        let vehicle2 = &mut ctx.accounts.vehicle2;
        
        if matches!(vehicle1.is_active, true) && matches!(vehicle2.is_active, true) {
            if new_route_code == 1 {
                vehicle1.current_route = Route::City;
                vehicle2.current_route = Route::City;
            } else {
                vehicle1.current_route = Route::Highway;
                vehicle2.current_route = Route::Highway;
            }
            msg!("Routes updated for both vehicles.");
        } else {
            msg!("One or both vehicles are inactive.");
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreateSystem<'info> {
    #[account(init, payer = manager, space = 8 + 32 + 8 + 4 + 1)]
    pub system: Account<'info, TrafficSystem>,
    #[account(mut)]
    pub manager: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct RegisterVehicle<'info> {
    #[account(mut, has_one = system)]
    pub system: Account<'info, TrafficSystem>,
    #[account(init, payer = owner, space = 8 + 32 + 4 + 1 + 1)]
    pub vehicle: Account<'info, Vehicle>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateRoutes<'info> {
    #[account(mut, has_one = system)]
    pub system: Account<'info, TrafficSystem>,
    #[account(mut, has_one = system)]
    pub vehicle1: Account<'info, Vehicle>,
    #[account(mut, has_one = system)]
    pub vehicle2: Account<'info, Vehicle>,
}

#[account]
pub struct TrafficSystem {
    pub manager: Pubkey,
    pub system_id: u64,
    pub vehicle_count: u32,
    pub is_operational: TrafficSystemStatus,
}

#[account]
pub struct Vehicle {
    pub system: Pubkey,
    pub vehicle_id: u32,
    pub is_active: bool,
    pub current_route: Route,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq)]
pub enum TrafficSystemStatus {
    Operational,
    Maintenance,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq)]
pub enum Route {
    City,
    Highway,
}