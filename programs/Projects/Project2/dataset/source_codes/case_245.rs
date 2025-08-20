use anchor_lang::prelude::*;
use std::collections::BTreeMap;

declare_id!("StakeMGR4040404040404040404040404040404040");

#[program]
pub mod nft_staking {
    use super::*;

    pub fn stake(ctx: Context<StakeNft>, nft_id: u64, amount: u64) -> Result<()> {
        let s = &mut ctx.accounts.stake;
        *s.stakes.entry(nft_id).or_insert(0) += amount;
        s.total_staked = s.total_staked.saturating_add(amount);
        Ok(())
    }

    pub fn unstake(ctx: Context<StakeNft>, nft_id: u64, amount: u64) -> Result<()> {
        let s = &mut ctx.accounts.stake;
        if let Some(v) = s.stakes.get_mut(&nft_id) {
            *v = v.saturating_sub(amount);
            s.total_staked = s.total_staked.saturating_sub(amount);
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct StakeNft<'info> {
    #[account(mut)]
    pub stake: Account<'info, StakeData>,
    pub user: Signer<'info>,
}

#[account]
pub struct StakeData {
    pub stakes: BTreeMap<u64, u64>,
    pub total_staked: u64,
}
