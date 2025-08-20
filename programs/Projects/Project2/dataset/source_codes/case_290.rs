use anchor_lang::prelude::*;

declare_id!("CompUpg999999999999999999999999999999999");

#[program]
pub mod component_upgrade {
    use super::*;

    pub fn upgrade(ctx: Context<Upgrade>, comp: u8) -> Result<()> {
        let u = &mut ctx.accounts.upgrade;
        if (comp as usize) < u.levels.len() {
            u.levels[comp as usize] = u.levels[comp as usize].saturating_add(1);
            u.count = u.count.saturating_add(1);
        } else {
            u.errors = u.errors.saturating_add(1);
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Upgrade<'info> {
    #[account(mut)]
    pub upgrade: Account<'info, UpgradeData>,
}

#[account]
pub struct UpgradeData {
    pub levels: [u8; 10],
    pub count: u64,
    pub errors: u64,
}
