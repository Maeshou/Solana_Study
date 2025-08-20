use anchor_lang::prelude::*;
use std::collections::BTreeMap;

declare_id!("SeasonR100100100100100100100100100100100");

#[program]
pub mod season_reward {
    use super::*;

    pub fn distribute(ctx: Context<Distribute>) -> Result<()> {
        let sd = &mut ctx.accounts.season;
        for (&player, &score) in sd.scores.iter() {
            let reward = score / sd.reward_divisor;
            sd.rewards.insert(player, reward);
        }
        sd.distributed = true;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Distribute<'info> {
    #[account(mut)]
    pub season: Account<'info, SeasonData>,
}

#[account]
pub struct SeasonData {
    pub scores: BTreeMap<Pubkey, u64>,
    pub rewards: BTreeMap<Pubkey, u64>,
    pub reward_divisor: u64,
    pub distributed: bool,
}
