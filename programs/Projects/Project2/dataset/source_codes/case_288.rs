use anchor_lang::prelude::*;
use std::collections::BTreeMap;

declare_id!("GuildTreas7777777777777777777777777777777");

#[program]
pub mod guild_treasury {
    use super::*;

    /// 分配計算
    pub fn distribute(ctx: Context<Distribute>) -> Result<()> {
        let t = &mut ctx.accounts.treasury;
        for (&member, &contrib) in t.contributions.iter() {
            let share = t.total_funds * contrib / t.total_contrib;
            t.shares.insert(member, share);
        }
        t.distributed = true;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Distribute<'info> {
    #[account(mut)]
    pub treasury: Account<'info, TreasuryData>,
}

#[account]
pub struct TreasuryData {
    pub contributions: BTreeMap<Pubkey, u64>,
    pub total_contrib: u64,
    pub total_funds: u64,
    pub shares: BTreeMap<Pubkey, u64>,
    pub distributed: bool,
}
