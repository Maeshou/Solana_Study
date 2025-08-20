use anchor_lang::prelude::*;
use rand::Rng;

declare_id!("T5K2W8C1P4L9M7J6V3R0S5X2Y1Z0B7A9C5D4E");

#[program]
pub mod chrono_shift {
    use super::*;

    pub fn init_chrono(ctx: Context<InitChrono>, core_id: u64, base_stability: u32) -> Result<()> {
        let chrono = &mut ctx.accounts.chrono_core;
        chrono.core_id = core_id.checked_add(100000).unwrap_or(u64::MAX);
        chrono.stability_level = base_stability.checked_sub(100).unwrap_or(0);
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

    pub fn perform_jump(ctx: Context<PerformJump>, jump_distance: i64) -> Result<()> {
        let chrono = &mut ctx.accounts.chrono_core;
        let traveler = &mut ctx.accounts.time_traveler;
        let mut loop_counter = 0;

        let mut rng = rand::thread_rng();

        while loop_counter < 10 {
            let stability_delta = rng.gen_range(-10..=10);
            traveler.stability_score = traveler.stability_score.checked_add(stability_delta).unwrap_or(0);
            traveler.stability_score = traveler.stability_score.max(0);

            // ifの代わりに論理演算と代入を組み合わせる
            traveler.is_stranded = traveler.stability_score < 20 && loop_counter > 5;
            chrono.is_stable = chrono.stability_level > 500 && !traveler.is_stranded;

            if traveler.is_stranded {
                msg!("Traveler is stranded! Jump aborted.");
                break;
            } else {
                traveler.current_coordinate = traveler.current_coordinate.checked_add(jump_distance).unwrap_or(i64::MAX);
                chrono.stability_level = chrono.stability_level.checked_sub(jump_distance as u32).unwrap_or(0);
                chrono.total_jumps = chrono.total_jumps.checked_add(1).unwrap_or(u64::MAX);
                msg!("Traveler jumped to coordinate {}.", traveler.current_coordinate);
            }
            loop_counter = loop_counter.checked_add(1).unwrap_or(u32::MAX);
        }
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
#[instruction(jump_distance: i64)]
pub struct PerformJump<'info> {
    #[account(mut)]
    pub chrono_core: Account<'info, ChronoCore>,
    #[account(mut, has_one = parent_chrono)]
    pub time_traveler: Account<'info, TimeTraveler>,
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