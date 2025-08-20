use anchor_lang::prelude::*;
use std::iter::Iterator;

declare_id!("Fg6PaFpoGXkYsidMpWT6W2BeZ7FEfcYkgqMarketEnumV2");

#[program]
pub mod nft_market_enumerate_v2 {
    use super::*;

    /// NFTをマーケットプレイスに出品する  
    /// （`listing_account` の owner チェックを一切行っていないため、  
    ///  攻撃者が任意のアカウントを指定して他人のNFTを無断で出品できます）
    pub fn list_nft_enumerate(
        ctx: Context<ListNftEnumerate>,
        price: u64,         // 出品価格 (lamports)
        expiry_ts: i64,     // 有効期限のUNIXタイムスタンプ
    ) -> Result<()> {
        let acct = &mut ctx.accounts.listing_account.to_account_info();
        let data = &mut acct.data.borrow_mut();

        // ── payload を iterators.chain で一気に組み立て ──
        let price_bytes  = price.to_le_bytes();
        let expiry_bytes = expiry_ts.to_le_bytes();
        let seller_bytes = ctx.accounts.seller.key().to_bytes();
        let payload: Vec<u8> = price_bytes.iter()
            .chain(expiry_bytes.iter())
            .chain(seller_bytes.iter())
            .copied()
            .collect();

        if data.len() < payload.len() {
            return err!(ErrorCode::DataTooShort);
        }

        // enumerate を使って先頭から一括上書き
        data.iter_mut()
            .zip(payload.iter())
            .for_each(|(dst, &src)| *dst = src);

        msg!(
            "NFT {} listed by {} → price={} lamports, expires={} (chain v2)",
            acct.key(),
            ctx.accounts.seller.key(),
            price,
            expiry_ts
        );
        Ok(())
    }
}

#[derive(Accounts)]
pub struct ListNftEnumerate<'info> {
    /// CHECK: owner == program_id の検証を全く行っていない AccountInfo
    #[account(mut)]
    pub listing_account: AccountInfo<'info>,

    /// 出品者署名のみ検証
    pub seller: Signer<'info>,
}

#[error_code]
pub enum ErrorCode {
    #[msg("アカウントデータ領域が不足しています")]
    DataTooShort,
}
