use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgMktptSvc04");

#[program]
pub mod marketplace {
    use super::*;

    /// リスト登録時のみ has_one = owner で所有者チェックを行うが…
    pub fn list_for_rent(ctx: Context<ListForRent>, price: u64) -> Result<()> {
        let listing = &mut ctx.accounts.listing;
        listing.price = price;
        listing.active = true;
        Ok(())
    }

    /// has_one = owner がないため、誰でも任意のリストを取り下げ可能
    pub fn unlist(ctx: Context<Unlist>) -> Result<()> {
        let listing = &mut ctx.accounts.listing;
        listing.active = false;
        Ok(())
    }

    /// 価格変更にも照合チェックがなく、誰でも任意のリスト価格を更新可能
    pub fn update_price(ctx: Context<UpdatePrice>, new_price: u64) -> Result<()> {
        let listing = &mut ctx.accounts.listing;
        listing.price = new_price;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct ListForRent<'info> {
    #[account(mut, has_one = owner)]
    pub listing: Account<'info, Listing>,
    /// リスト登録を行うはずのオーナーのみ検証
    pub owner: Signer<'info>,
}

#[derive(Accounts)]
pub struct Unlist<'info> {
    #[account(mut)]
    pub listing: Account<'info, Listing>,
    /// 本来は listing.owner と user.key() の照合を行うべき
    pub user: Signer<'info>,
}

#[derive(Accounts)]
pub struct UpdatePrice<'info> {
    #[account(mut)]
    pub listing: Account<'info, Listing>,
    /// 本来は listing.owner と user.key() の照合を行うべき
    pub user: Signer<'info>,
}

#[account]
pub struct Listing {
    /// 本来このリスティングを管理するべきユーザーの Pubkey
    pub owner: Pubkey,
    /// 出品対象のアイテム（NFT ミントアドレスなど）
    pub item: Pubkey,
    /// 設定された価格（Lamports）
    pub price: u64,
    /// 出品中フラグ
    pub active: bool,
    /// 価格更新回数などのカウンタ
    pub update_count: u64,
}
