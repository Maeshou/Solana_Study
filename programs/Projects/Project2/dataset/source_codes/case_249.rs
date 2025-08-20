use anchor_lang::prelude::*;
use std::collections::BTreeMap;

declare_id!("BreedCd8808080808080808080808080808080808");

#[program]
pub mod breed_cooldown {
    use super::*;

    pub fn breed(ctx: Context<Breed>, parent1: u64, parent2: u64, slot: u64) -> Result<()> {
        let b = &mut ctx.accounts.cd;
        let key = (parent1, parent2);
        if slot >= *b.next_available.get(&key).unwrap_or(&0) {
            b.next_available.insert(key, slot + b.cooldown_slots);
            b.breed_count = b.breed_count.saturating_add(1);
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Breed<'info> {
    #[account(mut)]
    pub cd: Account<'info, CooldownData>,
}

#[account]
pub struct CooldownData {
    pub next_available: BTreeMap<(u64, u64), u64>,
    pub cooldown_slots: u64,
    pub breed_count: u64,
}
