use anchor_lang::prelude::*;

// ── アカウントデータはファイル冒頭にタプル構造体で定義 ──
#[account]
#[derive(Default)]
pub struct ShoppingCart(pub u8, pub Vec<(u64, u64)>);

declare_id!("Fg6PaFpoGXkYsidMpWxTWq4rove6YjFgDhqyQ5RBwzV3");

#[program]
pub mod shopping_cart {
    use super::*;

    /// カート初期化：内部 Vec は空のまま、bump のみ設定
    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let bump = *ctx.bumps.get("cart").unwrap();
        ctx.accounts.cart.0 = bump;
        Ok(())
    }

    /// 商品追加：既存のエントリがあれば数量を加算、なければ新規追加
    pub fn add_item(ctx: Context<Modify>, item_id: u64, qty: u64) -> Result<()> {
        let items = &mut ctx.accounts.cart.1;
        let mut found = false;

        for entry in items.iter_mut() {
            if entry.0 == item_id {
                entry.1 = entry.1.wrapping_add(qty);
                found = true;
            }
        }
        if !found {
            items.push((item_id, qty));
        }
        Ok(())
    }

    /// 商品削除：既存エントリがあれば数量を減算、最後に残数0のものを除去
    pub fn remove_item(ctx: Context<Modify>, item_id: u64, qty: u64) -> Result<()> {
        let items = &mut ctx.accounts.cart.1;

        for entry in items.iter_mut() {
            if entry.0 == item_id {
                if entry.1 > qty {
                    entry.1 = entry.1.wrapping_sub(qty);
                } else {
                    entry.1 = 0;
                }
            }
        }
        items.retain(|&(_, q)| q > 0);
        Ok(())
    }

    /// カート全削除
    pub fn clear_cart(ctx: Context<Modify>) -> Result<()> {
        ctx.accounts.cart.1.clear();
        Ok(())
    }

    /// 総アイテム数報告：すべての数量を合計してログ出力
    pub fn count_items(ctx: Context<Modify>) -> Result<()> {
        let items = &ctx.accounts.cart.1;
        let mut total = 0u64;

        for &(_, q) in items.iter() {
            total = total.wrapping_add(q);
        }
        msg!("Total items in cart: {}", total);
        Ok(())
    }
}

// ── Context 定義は末尾に配置 ──
#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init_zeroed,
        payer = user,
        seeds = [b"cart", user.key().as_ref()],
        bump,
        // discriminator(8) + bump(1) + Vec<(u64,u64)> (max10件: 4 + 10*(8+8))
        space = 8 + 1 + 4 + 10 * (8 + 8)
    )]
    pub cart:   Account<'info, ShoppingCart>,

    #[account(mut)]
    pub user:   Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Modify<'info> {
    #[account(
        mut,
        seeds = [b"cart", user.key().as_ref()],
        bump = cart.0,
    )]
    pub cart:   Account<'info, ShoppingCart>,

    #[account(signer)]
    pub user:   AccountInfo<'info>,

    pub system_program: Program<'info, System>,
}
