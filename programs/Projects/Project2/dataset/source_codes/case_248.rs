use anchor_lang::prelude::*;
use std::collections::BTreeMap;

declare_id!("DailyQ7707070707070707070707070707070707");

#[program]
pub mod daily_quest {
    use super::*;

    pub fn try_quest(ctx: Context<TryQuest>, day_index: u64) -> Result<()> {
        let d = &mut ctx.accounts.daily;
        let cnt = d.attempts.entry(day_index).or_insert(0);
        if *cnt < d.max_per_day {
            *cnt = cnt.saturating_add(1);
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct TryQuest<'info> {
    #[account(mut)]
    pub daily: Account<'info, DailyData>,
    pub user: Signer<'info>,
}

#[account]
pub struct DailyData {
    pub attempts: BTreeMap<u64, u8>,
    pub max_per_day: u8,
}
