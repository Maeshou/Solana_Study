use anchor_lang::prelude::*;

declare_id!("MixInitMiss11111111111111111111111111111111");

#[program]
pub mod example1 {
    use super::*;

    // 新商品を登録する
    pub fn register_item(ctx: Context<RegisterItem>, price: u64) -> Result<()> {
        let item = &mut ctx.accounts.item;         // ← initあり
        item.price = price;

        let discount = &mut ctx.accounts.discount; // ← initなし（本来は初期化すべき）
        discount.percent = 0;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct RegisterItem<'info> {
    #[account(init, payer = user, space = 8 + 8)]
    pub item: Account<'info, ItemData>,
    pub discount: Account<'info, DiscountData>,
    #[account(mut)] pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct ItemData {
    pub price: u64,
}

#[account]
pub struct DiscountData {
    pub percent: u8,
}
