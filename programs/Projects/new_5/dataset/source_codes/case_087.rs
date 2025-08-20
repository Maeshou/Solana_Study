use anchor_lang::prelude::*;
use rand::Rng;

declare_id!("T5K2W8C1P4L9M7J6V3R0S5X2Y1Z0B7A9C5D4E");

#[program]
pub mod chrono_shift {
    use super::*;

    pub fn init_chrono(ctx: Context<InitChrono>, core_id: u64, base_stability: u32) -> Result<()> {
        let chrono = &mut ctx.accounts.chrono_core;
        chrono.core_id = core_id + 100000;
        chrono.stability_level = base_stability.saturating_sub(100);
        chrono.total_jumps = 0;
        chrono.is_stable = true;
        msg!("Chrono Core {} established with stability level {}.", chrono.core_id, chrono.stability_level);
        Ok(())
    }

    pub fn init_traveler(ctx: Context<InitTraveler>, traveler_id: u64, starting_coordinate: i64) -> Result<()> {
        let traveler = &mut ctx.accounts.time_traveler;
        traveler.parent_chrono = ctx.accounts.chrono_core.key();
        traveler.traveler_id = traveler_id ^ 0xFF00FF00FF00FF00;
        traveler.current_coordinate = starting_coordinate;
        traveler.stability_score = 100;
        traveler.is_stranded = false;
        msg!("Time Traveler {} registered at coordinate {}.", traveler.traveler_id, traveler.current_coordinate);
        Ok(())
    }

    pub fn perform_jump(ctx: Context<PerformJump>, jump_distance_1: i64, jump_distance_2: i64) -> Result<()> {
        let chrono = &mut ctx.accounts.chrono_core;
        let explorer_traveler = &mut ctx.accounts.explorer_traveler;
        let scout_traveler = &mut ctx.accounts.scout_traveler;
        let mut rng = rand::thread_rng();

        // explorer_travelerのジャンプ処理
        let stability_delta_1 = rng.gen_range(-10..=10);
        explorer_traveler.stability_score = explorer_traveler.stability_score.saturating_add(stability_delta_1).max(0);
        explorer_traveler.is_stranded = explorer_traveler.stability_score < 20;

        if !explorer_traveler.is_stranded {
            explorer_traveler.current_coordinate = explorer_traveler.current_coordinate.saturating_add(jump_distance_1);
            chrono.stability_level = chrono.stability_level.saturating_sub(jump_distance_1.abs() as u32);
            chrono.total_jumps = chrono.total_jumps.saturating_add(1);
        } else {
            msg!("Explorer traveler is stranded! Jump aborted.");
        }

        // scout_travelerのジャンプ処理
        let stability_delta_2 = rng.gen_range(-10..=10);
        scout_traveler.stability_score = scout_traveler.stability_score.saturating_add(stability_delta_2).max(0);
        scout_traveler.is_stranded = scout_traveler.stability_score < 20;

        if !scout_traveler.is_stranded {
            scout_traveler.current_coordinate = scout_traveler.current_coordinate.saturating_add(jump_distance_2);
            chrono.stability_level = chrono.stability_level.saturating_sub(jump_distance_2.abs() as u32);
            chrono.total_jumps = chrono.total_jumps.saturating_add(1);
        } else {
            msg!("Scout traveler is stranded! Jump aborted.");
        }

        chrono.is_stable = chrono.stability_level > 500 && !explorer_traveler.is_stranded && !scout_traveler.is_stranded;

        msg!("Explorer traveler jumped to coordinate {}. Scout traveler jumped to coordinate {}.", 
            explorer_traveler.current_coordinate, scout_traveler.current_coordinate);
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(core_id: u64, base_stability: u32)]
pub struct InitChrono<'info> {
    #[account(init, payer = signer, space = 8 + 8 + 4 + 8 + 1)]
    pub chrono_core: Account<'info, ChronoCore>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(traveler_id: u64, starting_coordinate: i64)]
pub struct InitTraveler<'info> {
    #[account(init, payer = signer, space = 8 + 32 + 8 + 8 + 4 + 1)]
    pub time_traveler: Account<'info, TimeTraveler>,
    #[account(mut)]
    pub chrono_core: Account<'info, ChronoCore>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(jump_distance_1: i64, jump_distance_2: i64)]
pub struct PerformJump<'info> {
    #[account(mut)]
    pub chrono_core: Account<'info, ChronoCore>,
    #[account(mut, has_one = parent_chrono)]
    pub explorer_traveler: Account<'info, TimeTraveler>,
    #[account(mut, has_one = parent_chrono)]
    pub scout_traveler: Account<'info, TimeTraveler>,
    pub signer: Signer<'info>,
}

#[account]
pub struct ChronoCore {
    core_id: u64,
    stability_level: u32,
    total_jumps: u64,
    is_stable: bool,
}

#[account]
pub struct TimeTraveler {
    parent_chrono: Pubkey,
    traveler_id: u64,
    current_coordinate: i64,
    stability_score: i32,
    is_stranded: bool,
}