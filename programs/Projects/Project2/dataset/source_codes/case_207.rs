use anchor_lang::prelude::*;
use std::collections::BTreeMap;

declare_id!("InvMap3333333333333333333333333333333333");

#[program]
pub mod inventory_map {
    use super::*;

    pub fn init(ctx: Context<InitMap>) -> Result<()> {
        let inv = &mut ctx.accounts.inv;
        inv.items = BTreeMap::new();
        Ok(())
    }

    pub fn add_item(ctx: Context<ModifyMap>, token_id: u64, qty: u32) -> Result<()> {
        let inv = &mut ctx.accounts.inv;
        let e = inv.items.entry(token_id).or_insert(0);
        *e = e.saturating_add(qty);
        Ok(())
    }

    pub fn use_item(ctx: Context<ModifyMap>, token_id: u64, qty: u32) -> Result<()> {
        let inv = &mut ctx.accounts.inv;
        if let Some(e) = inv.items.get_mut(&token_id) {
            *e = e.saturating_sub(qty);
            if *e == 0 {
                inv.items.remove(&token_id);
            }
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitMap<'info> {
    #[account(init, payer = user, space = 8 + 4 + (8 + 4) * 20)]
    pub inv: Account<'info, InvMapData>,
    #[account(mut)] pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ModifyMap<'info> {
    #[account(mut)] pub inv: Account<'info, InvMapData>,
}

#[account]
pub struct InvMapData {
    pub items: BTreeMap<u64, u32>,
}
