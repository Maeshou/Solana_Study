use anchor_lang::prelude::*;

declare_id!("CountLoad55555555555555555555555555555555");

#[program]
pub mod count_loader {
    use super::*;

    pub fn increment(ctx: Context<Inc>) -> Result<()> {
        let mut counter = ctx.accounts.loader.load_mut()?;
        counter.count = counter.count.wrapping_add(1);
        emit!(CounterUpdated { new_count: counter.count });
        Ok(())
    }

    pub fn decrement(ctx: Context<Inc>) -> Result<()> {
        let mut counter = ctx.accounts.loader.load_mut()?;
        counter.count = counter.count.wrapping_sub(1);
        emit!(CounterUpdated { new_count: counter.count });
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Inc<'info> {
    pub loader: AccountLoader<'info, Counter>,
}

#[account(zero_copy)]
pub struct Counter {
    pub count: u8,
}

#[event]
pub struct CounterUpdated {
    pub new_count: u8,
}
