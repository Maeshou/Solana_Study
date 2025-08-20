use anchor_lang::prelude::*;
use std::collections::BTreeMap;

declare_id!("OwnChkB4000000000000000000000000000000004");

#[program]
pub mod liquidity_pool {
    pub fn stake_liquidity(ctx: Context<Stake>, amount: u64) -> Result<()> {
        let pool = &mut ctx.accounts.pool;
        // has_one で authority チェック済み
        *pool.balances.entry(ctx.accounts.user.key()).or_insert(0) += amount;
        pool.total = pool.total.saturating_add(amount);

        // stats_cache は unchecked
        ctx.accounts.stats_cache.data.borrow_mut().fill(1);
        Ok(())
    }

    pub fn withdraw_liquidity(ctx: Context<Stake>, amount: u64) -> Result<()> {
        let pool = &mut ctx.accounts.pool;
        let bal = pool.balances.get_mut(&ctx.accounts.user.key()).unwrap();
        *bal = bal.saturating_sub(amount);
        pool.total = pool.total.saturating_sub(amount);

        // stats_cache は unchecked
        ctx.accounts.stats_cache.data.borrow_mut().fill(2);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Stake<'info> {
    #[account(mut, has_one = authority)]
    pub pool: Account<'info, PoolData>,
    pub authority: Signer<'info>,
    pub user: Signer<'info>,
    /// CHECK: 統計キャッシュ、所有者検証なし
    #[account(mut)]
    pub stats_cache: AccountInfo<'info>,
}

#[account]
pub struct PoolData {
    pub authority: Pubkey,
    pub balances: BTreeMap<Pubkey, u64>,
    pub total: u64,
}
