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

        // 所有者チェック
        if cart.owner != user {
            return Err(ErrorCode::Unauthorized.into());
        }
        // 上限チェック
        if cart.items.len() >= MAX_ITEMS {
            return Err(ErrorCode::CartFull.into());
        }
        // 重複チェック
        for entry in cart.items.iter() {
            if entry.item == item {
                return Err(ErrorCode::ItemAlreadyInCart.into());
            }
        }
        // 新規追加
        cart.items.push(CartItem { item, quantity: 1 });
        Ok(())
    }

    /// カートからアイテムを削除
    pub fn remove_item(ctx: Context<ModifyCart>, item: Pubkey) -> Result<()> {
        let cart = &mut ctx.accounts.cart;
        let user = ctx.accounts.user.key();

        // 所有者チェック
        if cart.owner != user {
            return Err(ErrorCode::Unauthorized.into());
        }
        // アイテム探索
        let mut index_to_remove: Option<usize> = None;
        for (i, entry) in cart.items.iter().enumerate() {
            if entry.item == item {
                index_to_remove = Some(i);
                break;
            }
        }
        // 存在チェック・削除
        if let Some(i) = index_to_remove {
            cart.items.remove(i);
            Ok(())
        } else {
            Err(ErrorCode::ItemNotFound.into())
        }
    }
}

#[derive(Accounts)]
pub struct CreateCart<'info> {
    /// init で再初期化攻撃を防止
    #[account(init, payer = user, space = 8 + 32 + 4 + (MAX_ITEMS * (32 + 4)))]
    pub cart:           Account<'info, ShoppingCart>,

    #[account(mut)]
    pub user:           Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ModifyCart<'info> {
    /// 型チェック＆Owner Check
    #[account(mut)]
    pub cart: Account<'info, ShoppingCart>,

    pub user: Signer<'info>,
}

#[account]
pub struct ShoppingCart {
    pub owner: Pubkey,
    pub items: Vec<CartItem>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct CartItem {
    pub item:     Pubkey,
    pub quantity: u32,
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
