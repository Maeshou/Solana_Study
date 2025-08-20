use anchor_lang::prelude::*;

declare_id!("MixInitMissLoop111111111111111111111111111");

#[program]
pub mod example1 {
    use super::*;

    // 新商品を初回登録（item にだけ init）
    pub fn register_item(ctx: Context<RegisterItem>, price: u64) -> Result<()> {
        let item = &mut ctx.accounts.item;
        item.price = price;
        Ok(())
    }

    // 割引率を設定（discount は init なし）
    pub fn set_discount(ctx: Context<SetDiscount>, threshold: u8) -> Result<()> {
        let item = &ctx.accounts.item;
        let discount = &mut ctx.accounts.discount;

        // 価格帯に応じて割引率を分岐
        if item.price > 1_000 {
            discount.percent = 20;
        } else {
            discount.percent = 5;
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct RegisterItem<'info> {
    #[account(init, payer = user, space = 8 + 8)]
    pub item: Account<'info, ItemData>,
    #[account(mut)] pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct SetDiscount<'info> {
    pub item: Account<'info, ItemData>,          // ← init なし：既存参照のみ
    pub discount: Account<'info, DiscountData>,  // ← init なし（本来は初期化すべき）
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
