use anchor_lang::prelude::*;
use std::collections::BTreeMap;

declare_id!("BreedLimit22222222222222222222222222222222");

#[program]
pub mod breeding_limit {
    use super::*;

    /// ブリーディング申請
    pub fn breed(ctx: Context<Breed>, parent1: u64, parent2: u64) -> Result<()> {
        let bd = &mut ctx.accounts.breed_data;
        let key = (parent1, parent2);
        let cnt = bd.user_breeds.entry(ctx.accounts.user.key()).or_insert(0);
        if *cnt < bd.max_per_user {
            *cnt = cnt.saturating_add(1);
            bd.pair_counts.entry(key).and_modify(|c| *c += 1).or_insert(1);
        } else {
            bd.blocked += 1;
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Breed<'info> {
    #[account(mut)]
    pub breed_data: Account<'info, BreedData>,
    pub user: Signer<'info>,
}

#[account]
pub struct BreedData {
    pub max_per_user: u64,
    pub user_breeds: BTreeMap<Pubkey, u64>,
    pub pair_counts: BTreeMap<(u64,u64), u64>,
    pub blocked: u64,
}
