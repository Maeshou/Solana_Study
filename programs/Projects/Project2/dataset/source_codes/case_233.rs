use anchor_lang::prelude::*;
use std::collections::BTreeMap;

declare_id!("AchvMap02222222222222222222222222222222222");

#[program]
pub mod achievement_tracker {
    use super::*;

    pub fn unlock(ctx: Context<UnlockAch>, ach_id: u8) -> Result<()> {
        let at = &mut ctx.accounts.ach;
        // 新規 or 更新：解除済みならカウントのみ増加
        at.map
            .entry(ach_id)
            .and_modify(|c| *c += 1)
            .or_insert(1);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct UnlockAch<'info> {
    #[account(mut)]
    pub ach: Account<'info, AchData>,
}

#[account]
pub struct AchData {
    pub map: BTreeMap<u8, u64>,
}
