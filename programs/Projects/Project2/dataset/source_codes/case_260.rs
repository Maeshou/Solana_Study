use anchor_lang::prelude::*;
use std::collections::BTreeMap;

declare_id!("BlkRec0999999999999999999999999999999999");

#[program]
pub mod recipe_block {
    use super::*;

    pub fn block(ctx: Context<Block>, recipe_id: u64, reason: String) -> Result<()> {
        let rb = &mut ctx.accounts.blocklist;
        rb.map.insert(recipe_id, reason);
        Ok(())
    }

    pub fn unblock(ctx: Context<Block>, recipe_id: u64, _dummy: String) -> Result<()> {
        let rb = &mut ctx.accounts.blocklist;
        rb.map.remove(&recipe_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Block<'info> {
    #[account(mut)]
    pub blocklist: Account<'info, BlockListData>,
    pub admin: Signer<'info>,
}

#[account]
pub struct BlockListData {
    pub map: BTreeMap<u64, String>,
}
