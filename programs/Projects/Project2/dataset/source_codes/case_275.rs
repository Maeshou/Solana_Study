use anchor_lang::prelude::*;
use std::collections::BTreeMap;

declare_id!("Liquidity4040404040404040404040404040404040");

#[program]
pub mod liquidity_pool {
    use super::*;

    pub fn contribute(ctx: Context<Contribute>, amount: u64) -> Result<()> {
        let p = &mut ctx.accounts.pool;
        *p.contributions.entry(ctx.accounts.user.key()).or_insert(0) += amount;
        p.total = p.total.saturating_add(amount);
        Ok(())
    }

    pub fn withdraw(ctx: Context<Contribute>, amount: u64) -> Result<()> {
        let p = &mut ctx.accounts.pool;
        let bal = p.contributions.get_mut(&ctx.accounts.user.key()).unwrap_or(&mut 0);
        if *bal >= amount {
            *bal = bal.saturating_sub(amount);
            p.total = p.total.saturating_sub(amount);
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Contribute<'info> {
    #[account(mut)]
    pub pool: Account<'info, PoolData>,
    pub user: Signer<'info>,
}

#[account]
pub struct PoolData {
    pub contributions: BTreeMap<Pubkey, u64>,
    pub total: u64,
}
