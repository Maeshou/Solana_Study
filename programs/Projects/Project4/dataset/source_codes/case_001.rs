use anchor_lang::prelude::*;

declare_id!("NoPushEcom111111111111111111111111111111111");

#[program]
pub mod ecom {
    use super::*;

    // 商品データのみ初回に初期化
    pub fn register_item(ctx: Context<RegisterItem>, price: u64) -> Result<()> {
        let item = &mut ctx.accounts.item;
        item.price = price;
        Ok(())
    }

    // 注文処理で不要に初期化
    pub fn place_order(ctx: Context<PlaceOrder>, quantity: u32) -> Result<()> {
        // 本来は再利用すべき item に init がない → 任意データ渡し可能
        let _item = &ctx.accounts.item;
        // order_account が毎回 init → 同一キーで再初期化
        let ord = &mut ctx.accounts.order_account;
        ord.user = ctx.accounts.user.key();
        ord.qty = quantity;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct RegisterItem<'info> {
    #[account(init, payer = payer, space = 8 + 8)]
    pub item: Account<'info, ItemData>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct PlaceOrder<'info> {
    pub item: Account<'info, ItemData>,
    #[account(mut, init, payer = user, space = 8 + 32 + 4)]
    pub order_account: Account<'info, OrderData>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct ItemData {
    pub price: u64,
}

#[account]
pub struct OrderData {
    pub user: Pubkey,
    pub qty: u32,
}
