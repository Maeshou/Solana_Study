use anchor_lang::prelude::*;
use std::collections::BTreeMap;

declare_id!("RentMkt2020202020202020202020202020202020");

#[program]
pub mod rental_market {
    use super::*;

    pub fn rent_out(ctx: Context<RentOut>, nft_id: u64, renter: Pubkey) -> Result<()> {
        let m = &mut ctx.accounts.market;
        m.current_rents.insert(nft_id, renter);
        *m.rental_counts.entry(nft_id).or_insert(0) += 1;
        Ok(())
    }

    pub fn reclaim(ctx: Context<Reclaim>, nft_id: u64) -> Result<()> {
        let m = &mut ctx.accounts.market;
        m.current_rents.remove(&nft_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct RentOut<'info> {
    #[account(mut)]
    pub market: Account<'info, RentalMarketData>,
    pub owner: Signer<'info>,
}

#[derive(Accounts)]
pub struct Reclaim<'info> {
    #[account(mut)]
    pub market: Account<'info, RentalMarketData>,
    pub owner: Signer<'info>,
}

#[account]
pub struct RentalMarketData {
    pub current_rents: BTreeMap<u64, Pubkey>,
    pub rental_counts: BTreeMap<u64, u64>,
}
