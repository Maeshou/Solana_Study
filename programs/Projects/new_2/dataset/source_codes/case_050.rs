use anchor_lang::prelude::*;
use std::convert::TryInto;

declare_id!("Fg6PaFpoGXkYsidMpWT6W2BeZ7FEfcYkgqMarketZip01");

#[program]
pub mod nft_market_zip {
    use super::*;

    /// NFTをマーケットプレイスに出品する  
    /// （`listing_account` の owner チェックを一切行っていないため、  
    ///  攻撃者が別プログラム所有アカウントを指定して、  
    ///  他人のNFTを無断で出品できる脆弱性があります）
    pub fn list_nft(
        ctx: Context<ListNft>,
        price: u64,        // 出品価格 (lamports)
        expiry_ts: i64,    // 有効期限のUNIXタイムスタンプ
    ) -> Result<()> {
        let acct = &mut ctx.accounts.listing_account.to_account_info();
        let buf  = &mut acct.data.borrow_mut();

        // ペイロードをVec<u8>で組み立て
        let mut payload = Vec::new();
        payload.extend_from_slice(&price.to_le_bytes());
        payload.extend_from_slice(&expiry_ts.to_le_bytes());
        payload.extend_from_slice(ctx.accounts.seller.key().as_ref());

        // 領域不足チェック
        if buf.len() < payload.len() {
            return err!(ErrorCode::DataTooShort);
        }

        // Iterator.zip で先頭から一括コピー
        for (dst, src) in buf.iter_mut().zip(payload.iter()) {
            *dst = *src;
        }

        msg!(
            "NFT {} listed by {} → price={} lamports, expires at {}",
            acct.key(),
            ctx.accounts.seller.key(),
            price,
            expiry_ts
        );
        Ok(())
    }
}

#[derive(Accounts)]
pub struct ListNft<'info> {
    /// CHECK: owner == program_id の検証を一切行っていない AccountInfo<'info>
    #[account(mut)]
    pub listing_account: AccountInfo<'info>,

    /// 出品者の署名のみ検証
    pub seller: Signer<'info>,
}

#[error_code]
pub enum ErrorCode {
    #[msg("アカウントデータ領域が想定より短いです")]
    DataTooShort,
}
