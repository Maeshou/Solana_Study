use anchor_lang::prelude::*;
use std::collections::BTreeMap;

declare_id!("InvVar0555555555555555555555555555555555");

#[program]
pub mod inventory_var5 {
    pub fn add_stock(ctx: Context<AddStock>, id: u64, qty: u32) -> Result<()> {
        let inv = &mut ctx.accounts.inv;
        // Anchor の has_one 属性で owner 検証
        inv.items.entry(id).and_modify(|q| *q += qty).or_insert(qty);
        inv.total = inv.total.saturating_add(qty as u64);

        // backup_log は unchecked
        let mut buf = ctx.accounts.backup_log.data.borrow_mut();
        buf.extend_from_slice(&id.to_le_bytes());
        Ok(())
    }
}

#[derive(Accounts)]
pub struct AddStock<'info> {
    #[account(mut, has_one = owner)]
    pub inv: Account<'info, InvData>,
    pub owner: Signer<'info>,
    #[account(mut)] pub backup_log: AccountInfo<'info>,  // unchecked
}

#[account]
pub struct InvData {
    pub owner: Pubkey,
    pub items: BTreeMap<u64, u32>,
    pub total: u64,
}
