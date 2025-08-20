use anchor_lang::prelude::*;

declare_id!("PricExa9999999999999999999999999999999999");

#[program]
pub mod price_extra {
    use super::*;

    pub fn adjust(ctx: Context<ModifyPrice>, delta: i64) -> Result<()> {
        let p = &mut ctx.accounts.price;
        if delta >= 0 {
            p.current = (p.current as i64 + delta) as u64;
            p.adjust_count = p.adjust_count.saturating_add(1);
        } else {
            // 負の調整時は復元処理
            let restore = (-delta) as u64;
            p.current = p.current.saturating_add(restore);
            p.restore_count = p.restore_count.saturating_add(1);
            p.last_restored = restore;
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct ModifyPrice<'info> {
    #[account(mut)]
    pub price: Account<'info, PriceExtraData>,
}

#[account]
pub struct PriceExtraData {
    pub current: u64,
    pub adjust_count: u64,
    pub restore_count: u64,
    pub last_restored: u64,
}
