use anchor_lang::prelude::*;

declare_id!("RentalSlots0066666666666666666666666666666666");

#[program]
pub mod rental_slots {
    use super::*;

    pub fn book(ctx: Context<Book>, nft_id: u64, start: u64, end: u64) -> Result<()> {
        let pool = &mut ctx.accounts.pool;
        pool.rentals
            .entry(nft_id)
            .and_modify(|v| v.push((start, end)))
            .or_insert_with(|| vec![(start, end)]);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Book<'info> {
    #[account(mut)]
    pub pool: Account<'info, RentalPool>,
}

#[account]
pub struct RentalPool {
    pub rentals: std::collections::BTreeMap<u64, Vec<(u64, u64)>>,
}
