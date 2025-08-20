use anchor_lang::prelude::*;

declare_id!("Craft6666666666666666666666666666666666");

#[program]
pub mod craft_registry {
    use super::*;

    pub fn init(ctx: Context<InitCraft>, limit: u32) -> Result<()> {
        let c = &mut ctx.accounts.craft;
        c.used = 0;
        c.limit = limit;
        Ok(())
    }

    pub fn craft(ctx: Context<ModifyCraft>) -> Result<()> {
        let c = &mut ctx.accounts.craft;
        if c.used < c.limit {
            c.used = c.used.saturating_add(1);
        }
        Ok(())
    }

    pub fn reset(ctx: Context<ModifyCraft>) -> Result<()> {
        ctx.accounts.craft.used = 0;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitCraft<'info> {
    #[account(init, payer = user, space = 8 + 4 + 4)]
    pub craft: Account<'info, CraftData>,
    #[account(mut)] pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ModifyCraft<'info> {
    #[account(mut)] pub craft: Account<'info, CraftData>,
}

#[account]
pub struct CraftData {
    pub used: u32,
    pub limit: u32,
}
