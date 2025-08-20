// ============================================================================
// 2) Ride Booking（配車） — constraint 連鎖 + require!
// ============================================================================
declare_id!("RB22222222222222222222222222222222");

#[program]
pub mod ride_booking {
    use super::*;

    pub fn init_order(ctx: Context<InitOrder>, dist: u32, base: u64) -> Result<()> {
        ctx.accounts.order.rider = ctx.accounts.rider.key();
        ctx.accounts.order.distance = dist;
        ctx.accounts.order.fare = base;
        ctx.accounts.order.active = true;

        ctx.accounts.driver.driver = ctx.accounts.captain.key();
        ctx.accounts.driver.rating = 5;
        ctx.accounts.driver.trips = 0;
        ctx.accounts.driver.on_duty = true;

        ctx.accounts.city.max_distance = 500;
        ctx.accounts.city.surge = 1;
        ctx.accounts.city.bump = *ctx.bumps.get("city").unwrap();
        Ok(())
    }

    pub fn adjust_fare(ctx: Context<AdjustFare>, step: u32) -> Result<()> {
        // order != driver, order != city は attribute 側で担保。ここで driver != city を担保。
        require!(ctx.accounts.driver.key() != ctx.accounts.city.key(), ProgramError::InvalidArgument);

        for _ in 0..step {
            ctx.accounts.order.fare = ctx.accounts.order.fare.saturating_add(1);
            ctx.accounts.driver.trips = ctx.accounts.driver.trips.saturating_add(1);
        }

        if ctx.accounts.order.distance > ctx.accounts.city.max_distance {
            ctx.accounts.order.active = false;
            ctx.accounts.driver.on_duty = false;
            ctx.accounts.city.surge = ctx.accounts.city.surge.saturating_add(1);
            msg!("too far: dist={} max={}", ctx.accounts.order.distance, ctx.accounts.city.max_distance);
        } else {
            ctx.accounts.order.active = true;
            ctx.accounts.driver.on_duty = true;
            ctx.accounts.city.surge = ctx.accounts.city.surge.saturating_add(0);
            msg!("ok: fare={} surge={}", ctx.accounts.order.fare, ctx.accounts.city.surge);
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitOrder<'info> {
    #[account(init, payer = payer, space = 8 + 32 + 4 + 8 + 1)]
    pub order: Account<'info, RideOrder>,
    #[account(init, payer = payer, space = 8 + 32 + 1 + 4)]
    pub driver: Account<'info, DriverProfile>,
    #[account(init, seeds = [b"city", payer.key().as_ref()], bump, payer = payer, space = 8 + 4 + 1 + 1)]
    pub city: Account<'info, CityRules>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub rider: Signer<'info>,
    pub captain: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct AdjustFare<'info> {
    #[account(mut, constraint = order.key() != driver.key(), error = RideErr::Dup)]
    pub order: Account<'info, RideOrder>,
    #[account(mut, constraint = order.key() != city.key(), error = RideErr::Dup)]
    pub driver: Account<'info, DriverProfile>,
    #[account(mut)]
    pub city: Account<'info, CityRules>,
}

#[account] pub struct RideOrder { pub rider: Pubkey, pub distance: u32, pub fare: u64, pub active: bool }
#[account] pub struct DriverProfile { pub driver: Pubkey, pub on_duty: bool, pub rating: u8, pub trips: u32 }
#[account] pub struct CityRules { pub max_distance: u32, pub surge: u8, pub bump: u8 }

#[error_code] pub enum RideErr { #[msg("duplicate mutable account")] Dup }

