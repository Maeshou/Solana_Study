use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgSaleIntv001");

#[program]
pub mod sale_interface {
    use super::*;

    /// リスティング価格を更新するが、所有者照合チェックがない
    pub fn set_price(ctx: Context<SetPrice>, new_price: u64) -> Result<()> {
        let listing = &mut ctx.accounts.listing;
        listing.price = new_price;
        listing.update_count = listing.update_count.checked_add(1).unwrap();
        Ok(())
    }

    /// 出品を終了するが、所有者照合チェックがない
    pub fn close_sale(ctx: Context<CloseSale>) -> Result<()> {
        let listing = &mut ctx.accounts.listing;
        listing.is_active = false;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct SetPrice<'info> {
    #[account(mut)]
    /// 本来は #[account(has_one = owner)] を指定して所有者一致を検証すべき
    pub listing: Account<'info, Listing>,
    /// 価格更新をリクエストするユーザー（署名者）
    pub user: Signer<'info>,
}

#[derive(Accounts)]
pub struct CloseSale<'info> {
    #[account(mut)]
    /// 本来は #[account(has_one = owner)] を指定して所有者一致を検証すべき
    pub listing: Account<'info, Listing>,
    /// 出品終了をリクエストするユーザー（署名者）
    pub user: Signer<'info>,
}

#[account]
pub struct Listing {
    /// 本来このリスティングを所有するべきユーザーの Pubkey
    pub owner: Pubkey,
    /// 設定された販売価格
    pub price: u64,
    /// 販売中かどうか
    pub is_active: bool,
    /// 設定価格の更新回数
    pub update_count: u64,
}
