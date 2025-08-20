use anchor_lang::prelude::*;
use std::collections::BTreeMap;

declare_id!("FuseHist00111111111111111111111111111111");

#[program]
pub mod nft_fusion {
    use super::*;

    pub fn fuse(ctx: Context<Fuse>, parent_a: u64, parent_b: u64, result: u64) -> Result<()> {
        let data = &mut ctx.accounts.history;
        let key = (parent_a.min(parent_b), parent_a.max(parent_b));
        data.successes.insert(key, result);
        *data.fusion_count.entry(key).or_insert(0) += 1;
        Ok(())
    }

    pub fn fail_fuse(ctx: Context<Fuse>, parent_a: u64, parent_b: u64) -> Result<()> {
        let data = &mut ctx.accounts.history;
        let key = (parent_a.min(parent_b), parent_a.max(parent_b));
        *data.fail_count.entry(key).or_insert(0) += 1;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Fuse<'info> {
    #[account(mut)]
    pub history: Account<'info, FusionHistory>,
}

#[account]
pub struct FusionHistory {
    pub successes: BTreeMap<(u64, u64), u64>,
    pub fusion_count: BTreeMap<(u64, u64), u64>,
    pub fail_count: BTreeMap<(u64, u64), u64>,
}
