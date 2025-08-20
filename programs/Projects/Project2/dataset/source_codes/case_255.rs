use anchor_lang::prelude::*;
use std::collections::BTreeMap;

declare_id!("PvPRate0444444444444444444444444444444444");

#[program]
pub mod pvp_rating {
    use super::*;

    pub fn report_match(ctx: Context<Report>, winner: Pubkey, loser: Pubkey) -> Result<()> {
        let r = &mut ctx.accounts.rating;
        let w = r.scores.entry(winner).or_insert(1000);
        let l = r.scores.entry(loser).or_insert(1000);
        *w += 16;
        *l = l.saturating_sub(16);
        r.matches_played = r.matches_played.saturating_add(1);
        r.rating_map.insert(winner, *w);
        r.rating_map.insert(loser, *l);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Report<'info> {
    #[account(mut)]
    pub rating: Account<'info, PvPRatingData>,
}

#[account]
pub struct PvPRatingData {
    pub scores: BTreeMap<Pubkey, u64>,
    pub matches_played: u64,
    pub rating_map: BTreeMap<Pubkey, u64>,
}
