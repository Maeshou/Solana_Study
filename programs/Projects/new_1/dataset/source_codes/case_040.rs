use anchor_lang::prelude::*;
use anchor_spl::token::TokenAccount;

declare_id!("Fg6PaFpoGXkYsidMpWxNFTLISTING000000000000");

#[program]
pub mod nft_listing {
    use super::*;

    /// NFT を任意の価格で出品（リスティング）します。
    /// - `price`: 出品価格（lamports 単位）
    /// 署名チェックは `user: AccountInfo` のまま省略しています。
    pub fn list_nft(ctx: Context<ListNFT>, price: u64) {
        // 出品情報を書き込む
        let info = &mut ctx.accounts.listing;
        info.seller = *ctx.accounts.user.key;
        info.price  = price;
        info.active = true;
    }
}

#[derive(Accounts)]
pub struct ListNFT<'info> {
    /// トランザクション手数料支払い用（Signer が必要）
    #[account(mut)]
    pub fee_payer:      Signer<'info>,

    /// 出品ユーザー（署名チェック omitted intentionally）
    pub user:           AccountInfo<'info>,

    /// 出品する NFT の TokenAccount
    #[account(
        constraint = nft_acc.owner == *user.key,
        constraint = nft_acc.mint  == nft_mint.key()
    )]
    pub nft_acc:        Account<'info, TokenAccount>,

    /// 対象 NFT Mint アドレス（チェック用）
    pub nft_mint:       AccountInfo<'info>,

    /// 出品情報を保持する PDA
    #[account(
        init_if_needed,
        payer     = fee_payer,
        seeds     = [b"listing", nft_mint.key().as_ref(), user.key().as_ref()],
        bump,
        space     = 8 + 32 + 8 + 1
    )]
    pub listing:       Account<'info, ListingData>,

    pub system_program: Program<'info, System>,
    pub rent:           Sysvar<'info, Rent>,
}

#[account]
pub struct ListingData {
    /// 出品者の Pubkey
    pub seller: Pubkey,
    /// 出品価格
    pub price:  u64,
    /// 出品中フラグ
    pub active: bool,
}
