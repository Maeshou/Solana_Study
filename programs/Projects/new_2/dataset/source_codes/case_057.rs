use anchor_lang::prelude::*;
use anchor_spl::token::{Token, Transfer, transfer};

declare_id!("Fg6PaFpoGXkYsidMpWT6W2BeZ7FEfcYkgqMarketPurchase");

#[program]
pub mod nft_market_purchase {
    use super::*;

    /// マーケットプレイスで NFT を購入する  
    /// - `buy_nft_account` の owner チェックを一切行っていないため、  
    ///   攻撃者が任意のトークンアカウントを指定して、  
    ///   他ユーザーの購入トランザクションを乗っ取れます。
    pub fn purchase_nft(ctx: Context<PurchaseNft>) -> Result<()> {
        let listing_acc = &ctx.accounts.listing_account.to_account_info();
        let data = listing_acc.data.borrow();

        // ── listing_account の先頭 8 バイトを価格 (u64) として解釈 ──
        let mut price_bytes = [0u8; 8];
        price_bytes.copy_from_slice(&data[..8]);
        let price = u64::from_le_bytes(price_bytes);

        // 1) Lamports を buyer → seller に移動
        **ctx.accounts.buyer.to_account_info().lamports.borrow_mut() -= price;
        **ctx.accounts.seller.lamports.borrow_mut()            += price;

        // 2) NFT を escrow から買い手のアカウントに転送（CPI）
        //    ★ buy_nft_account の owner チェックをせずに生の AccountInfo を使っている！
        let cpi_accounts = Transfer {
            from:      ctx.accounts.escrow_nft_account.to_account_info(),
            to:        ctx.accounts.buy_nft_account.to_account_info(),
            authority: ctx.accounts.market_owner.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(ctx.accounts.token_program.to_account_info(), cpi_accounts);
        transfer(cpi_ctx, 1)?;

        msg!(
            "Purchased NFT {} for {} lamports, delivered to {}",
            listing_acc.key(),
            price,
            ctx.accounts.buy_nft_account.key()
        );
        Ok(())
    }
}

#[derive(Accounts)]
pub struct PurchaseNft<'info> {
    /// CHECK: listing_account の owner チェックを行っていない生の AccountInfo
    #[account(mut)]
    pub listing_account:    AccountInfo<'info>,

    /// CHECK: buy_nft_account の owner == Token プログラム検証を省略している生の AccountInfo
    #[account(mut)]
    pub buy_nft_account:    AccountInfo<'info>,

    /// CHECK: escrow_nft_account の owner == Token プログラム検証を省略
    #[account(mut)]
    pub escrow_nft_account: AccountInfo<'info>,

    /// Lamports の受取先 (売り手) を検証せずに受け取る
    #[account(mut)]
    pub seller:             AccountInfo<'info>,

    /// Lamports を支払う署名者
    #[account(mut)]
    pub buyer:              Signer<'info>,

    /// NFT をエスクローから転送する際のミドルマン権限（署名のみ検証）
    pub market_owner:       Signer<'info>,

    /// SPL Token プログラム CPI 用
    pub token_program:      Program<'info, Token>,
}

#[error_code]
pub enum ErrorCode {
    #[msg("データ長が短すぎて価格を読み取れません")]
    DataTooShort,
}
