use anchor_lang::prelude::*;
use std::collections::BTreeMap;

declare_id!("InvExa6666666666666666666666666666666666");

#[program]
pub mod inv_limit {
    use super::*;

    pub fn modify(ctx: Context<ModifyInv>, token_id: u64) -> Result<()> {
        let inv = &mut ctx.accounts.inv;
        if let Some(qty) = inv.items.get_mut(&token_id) {
            if *qty > 0 {
                // 使用
                *qty -= 1;
                inv.uses = inv.uses.saturating_add(1);
            } else {
                // 在庫ゼロ時の処理
                inv.out_of_stock = inv.out_of_stock.saturating_add(1);
                inv.warn_count = inv.warn_count.saturating_add(1);
            }
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct ModifyInv<'info> {
    #[account(mut)]
    pub inv: Account<'info, InvLimitData>,
}

#[account]
pub struct InvLimitData {
    pub items: BTreeMap<u64, u32>,
    pub uses: u64,
    pub out_of_stock: u64,
    pub warn_count: u64,
}
