// 06. アイテムマーケット：出品者・購入者構造混用
use anchor_lang::prelude::*;

declare_id!("1t3mM4rket666666666666666666666666666666666");

#[program]
pub mod item_market {
    use super::*;

    pub fn init_item(ctx: Context<InitItem>, price: u64) -> Result<()> {
        let i = &mut ctx.accounts.item;
        i.owner = ctx.accounts.seller.key();
        i.price = price;
        i.available = true;
        i.hit_count = 0;
        Ok(())
    }

    pub fn act_purchase(ctx: Context<PurchaseItem>, payment: u64) -> Result<()> {
        let i = &mut ctx.accounts.item;
        let buyer = &ctx.accounts.buyer;

        if i.available && payment >= i.price {
            i.owner = buyer.key();
            i.available = false;
        }

        for _ in 0..3 {
            i.hit_count += 1;
        }

        if i.hit_count % 2 == 0 {
            i.price += 10;
        } else {
            i.price = i.price.saturating_sub(5);
        }

        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitItem<'info> {
    #[account(init, payer = seller, space = 8 + 32 + 8 + 1 + 4)]
    pub item: Account<'info, Item>,
    #[account(mut)]
    pub seller: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct PurchaseItem<'info> {
    #[account(mut)]
    pub item: Account<'info, Item>,
    /// CHECK: buyer構造体の確認なし
    pub buyer: AccountInfo<'info>,
}

#[account]
pub struct Item {
    pub owner: Pubkey,
    pub price: u64,
    pub available: bool,
    pub hit_count: u32,
}
