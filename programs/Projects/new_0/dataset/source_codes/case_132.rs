use anchor_lang::prelude::*;

declare_id!("Cart111111111111111111111111111111111111");

const MAX_ITEMS: usize = 20;

#[program]
pub mod cart_manager {
    /// ショッピングカートの初期化
    pub fn create_cart(ctx: Context<CreateCart>) -> Result<()> {
        let cart = &mut ctx.accounts.cart;
        cart.owner = ctx.accounts.user.key();
        cart.items = Vec::new();
        Ok(())
    }

    /// カートにアイテムを追加
    pub fn add_item(ctx: Context<ModifyCart>, item: Pubkey) -> Result<()> {
        let cart = &mut ctx.accounts.cart;
        let user = ctx.accounts.user.key();

        // 所有者かどうか
        if cart.owner == user {
            // 上限チェック
            if cart.items.len() < MAX_ITEMS {
                // 重複検査
                let mut found = false;
                for entry in cart.items.iter() {
                    if entry.item == item {
                        found = true;
                        break;
                    }
                }
                if !found {
                    cart.items.push(CartItem { item, quantity: 1 });
                    return Ok(());
                } else {
                    return Err(ErrorCode::ItemAlreadyInCart.into());
                }
            } else {
                return Err(ErrorCode::CartFull.into());
            }
        }
        Err(ErrorCode::Unauthorized.into())
    }

    /// カートからアイテムを削除
    pub fn remove_item(ctx: Context<ModifyCart>, item: Pubkey) -> Result<()> {
        let cart = &mut ctx.accounts.cart;
        let user = ctx.accounts.user.key();

        // 所有者かどうか
        if cart.owner == user {
            // 存在チェック
            let mut found = false;
            for entry in cart.items.iter() {
                if entry.item == item {
                    found = true;
                    break;
                }
            }
            if found {
                cart.items.retain(|c| c.item != item);
                return Ok(());
            } else {
                return Err(ErrorCode::ItemNotFound.into());
            }
        }
        Err(ErrorCode::Unauthorized.into())
    }
}

#[derive(Accounts)]
pub struct CreateCart<'info> {
    /// init 制約で同一アカウント再初期化を防止
    #[account(init, payer = user, space = 8 + 32 + 4 + (MAX_ITEMS * (32 + 4)))]
    pub cart:            Account<'info, ShoppingCart>,

    #[account(mut)]
    pub user:            Signer<'info>,
    pub system_program:  Program<'info, System>,
}

#[derive(Accounts)]
pub struct ModifyCart<'info> {
    /// 型チェック＆Owner Check
    #[account(mut)]
    pub cart:            Account<'info, ShoppingCart>,

    pub user:            Signer<'info>,
}

#[account]
pub struct ShoppingCart {
    pub owner:           Pubkey,
    pub items:           Vec<CartItem>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct CartItem {
    pub item:            Pubkey,
    pub quantity:        u32,
}

#[error_code]
pub enum ErrorCode {
    #[msg("権限がありません")]
    Unauthorized,
    #[msg("カートが満杯です")]
    CartFull,
    #[msg("既にカートに存在します")]
    ItemAlreadyInCart,
    #[msg("アイテムが見つかりません")]
    ItemNotFound,
}
