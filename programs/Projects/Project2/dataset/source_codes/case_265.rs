use anchor_lang::prelude::*;
use std::collections::BTreeMap;

declare_id!("Synergy044444444444444444444444444444444");

#[program]
pub mod synergy_registry {
    use super::*;

    pub fn register(ctx: Context<Register>, a: u64, b: u64, bonus: u64) -> Result<()> {
        let sr = &mut ctx.accounts.registry;
        let inner = sr.map.entry(a).or_insert_with(BTreeMap::new);
        inner.insert(b, bonus);
        Ok(())
    }

    pub fn query_bonus(ctx: Context<Query>, a: u64, b: u64) -> Result<Bonus> {
        let sr = &ctx.accounts.registry;
        let bonus = sr
            .map
            .get(&a)
            .and_then(|m| m.get(&b))
            .copied()
            .unwrap_or(0);
        Ok(Bonus { bonus })
    }
}

#[derive(Accounts)]
pub struct Register<'info> {
    #[account(mut)]
    pub registry: Account<'info, SynergyData>,
}

#[derive(Accounts)]
pub struct Query<'info> {
    pub registry: Account<'info, SynergyData>,
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct Bonus {
    pub bonus: u64,
}

#[account]
pub struct SynergyData {
    pub map: BTreeMap<u64, BTreeMap<u64, u64>>,
}
