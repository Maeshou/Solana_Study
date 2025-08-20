use anchor_lang::prelude::*;

declare_id!("CountLoad55555555555555555555555555555555");

#[program]
pub mod count_loader {
    use super::*;

    pub fn increment(ctx: Context<Inc>) -> Result<()> {
        let mut d = ctx.accounts.loader.load_mut()?;
        d.count = d.count.wrapping_add(1);
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
