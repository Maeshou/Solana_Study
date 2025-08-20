use anchor_lang::prelude::*;
use anchor_spl::token::Mint;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgRentMk01");

#[program]
pub mod rent_marketplace {
    use super::*;

    /// NFT をレンタルリストに登録するが、
    /// listing.owner と ctx.accounts.user.key() の照合検証がない
    pub fn list_for_rent(ctx: Context<ListForRent>, price: u64) -> Result<()> {
        let listing = &mut ctx.accounts.listing;
        // 本来は #[account(has_one = owner)] で listing.owner と user.key() を検証すべき
        listing.nft_mint = ctx.accounts.nft_mint.key();
        listing.price = price;
        listing.active = true;
        Ok(())
    }

    /// レンタルリストから削除するが、
    /// listing.owner と ctx.accounts.user.key() の照合検証がない
    pub fn unlist(ctx: Context<Unlist>) -> Result<()> {
        let listing = &mut ctx.accounts.listing;
        // 本来は #[account(has_one = owner)] で listing.owner と user.key() を検証すべき
        listing.active = false;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct ListForRent<'info> {
    #[account(mut)]
    /// 本来は #[account(has_one = owner)] を指定して所有者照合を行うべき
    pub listing: Account<'info, Listing>,
    /// リストに登録する NFT のミントアカウント
    pub nft_mint: Account<'info, Mint>,
    /// リスト登録をリクエストするユーザー（署名者）
    pub user: Signer<'info>,
}

#[derive(Accounts)]
pub struct Unlist<'info> {
    #[account(mut)]
    /// 本来は #[account(has_one = owner)] を指定して所有者照合を行うべき
    pub listing: Account<'info, Listing>,
    /// リスト削除をリクエストするユーザー（署名者）
    pub user: Signer<'info>,
}

#[account]
pub struct Listing {
    /// 本来このリスティングを所有するべきユーザーの Pubkey
    pub owner: Pubkey,
    /// レンタル対象の NFT ミントアドレス
    pub nft_mint: Pubkey,
    /// レンタル価格（Lamports）
    pub price: u64,
    /// 出品中フラグ
    pub active: bool,
}
