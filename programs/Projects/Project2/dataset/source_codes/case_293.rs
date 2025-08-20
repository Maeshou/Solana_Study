use anchor_lang::prelude::*;
use std::collections::BTreeMap;

declare_id!("AchvNFT0022222222222222222222222222222222");

#[program]
pub mod nft_achievements {
    use super::*;

    pub fn unlock(ctx: Context<Unlock>, nft_id: u64, name: String) -> Result<()> {
        let tracker = &mut ctx.accounts.tracker;
        tracker.achievements
            .entry(nft_id)
            .and_modify(|v| if !v.contains(&name) { v.push(name.clone()) })
            .or_insert_with(|| vec![name.clone()]);
        Ok(())
    }

    pub fn clear(ctx: Context<Unlock>, nft_id: u64) -> Result<()> {
        let tracker = &mut ctx.accounts.tracker;
        tracker.achievements.remove(&nft_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Unlock<'info> {
    #[account(mut)]
    pub tracker: Account<'info, AchievementTracker>,
}

#[account]
pub struct AchievementTracker {
    pub achievements: BTreeMap<u64, Vec<String>>,
}
