use anchor_lang::prelude::*;
use std::collections::BTreeMap;

declare_id!("GuildBk0333333333333333333333333333333333");

#[program]
pub mod guild_bank {
    use super::*;

    pub fn deposit(ctx: Context<Transact>, amount: u64) -> Result<()> {
        let b = &mut ctx.accounts.bank;
        *b.balances.entry(ctx.accounts.user.key()).or_insert(0) += amount;
        b.total = b.total.saturating_add(amount);
        Ok(())
    }

    pub fn withdraw(ctx: Context<Transact>, amount: u64) -> Result<()> {
        let b = &mut ctx.accounts.bank;
        let bal = b.balances.get_mut(&ctx.accounts.user.key()).unwrap_or(&mut 0);
        if *bal >= amount {
            *bal = bal.saturating_sub(amount);
            b.total = b.total.saturating_sub(amount);
        } else {
            b.failed_withdrawals = b.failed_withdrawals.saturating_add(1);
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Transact<'info> {
    #[account(mut)]
    pub bank: Account<'info, GuildBankData>,
    pub user: Signer<'info>,
}

#[account]
pub struct GuildBankData {
    pub balances: BTreeMap<Pubkey, u64>,
    pub total: u64,
    pub failed_withdrawals: u64,
}
