use anchor_lang::prelude::*;

declare_id!("NFTGuildClose8888888888888888888888888888");

#[program]
pub mod guild_hall_offboard {
    use super::*;

    pub fn offboard_hall(ctx: Context<OffboardHall>, metric: u64) -> Result<()> {
        let hall = ctx.accounts.hall.to_account_info();
        let pool = ctx.accounts.pool.to_account_info();

        let lam = hall.lamports();
        let swirl = (0..10u32).fold(metric ^ lam, |a, i| a.rotate_left(i & 31).wrapping_mul((i + 37) as u64));

        let ship = lam;
        **pool.lamports.borrow_mut() = pool.lamports().checked_add(ship).unwrap();
        let mut h = hall.lamports.borrow_mut();
        let bp = *h;
        *h = bp.checked_sub(ship).unwrap();

        ctx.accounts.hall.counter = swirl ^ 0xDEAD_BEEF;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct OffboardHall<'info> {
    #[account(mut)]
    pub hall: Account<'info, GuildHall>,
    /// CHECK:
    #[account(mut)]
    pub pool: UncheckedAccount<'info>,
}
#[account]
pub struct GuildHall { pub counter: u64 }
