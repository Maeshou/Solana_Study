use anchor_lang::prelude::*;
use std::collections::BTreeMap;

declare_id!("ResHarvest077777777777777777777777777777777");

#[program]
pub mod resource_harvest {
    use super::*;

    pub fn harvest(ctx: Context<Harvest>, node_id: u8) -> Result<()> {
        let rh = &mut ctx.accounts.res;
        rh.counts.insert(node_id, rh.counts.get(&node_id).copied().unwrap_or(0) + 1);
        if rh.counts.len() > 10 {
            let smallest = *rh.counts.keys().min().unwrap();
            rh.counts.remove(&smallest);
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Harvest<'info> {
    #[account(mut)]
    pub res: Account<'info, ResourceData>,
}

#[account]
pub struct ResourceData {
    pub counts: BTreeMap<u8, u64>,
}
