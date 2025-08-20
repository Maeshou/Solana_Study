use anchor_lang::prelude::*;
use bytemuck::{Pod, Zeroable};

declare_id!("Fg6PaFpoGXkYsidMpWT6W2BeZ7FEfcYkgqMarketV2XYZ");

#[repr(C)]
#[derive(Copy, Clone, Pod, Zeroable)]
struct ListingData {
    pub price:   u64,
    pub fee_bps: u16,
    pub expiry:  u64,
    pub seller:  [u8; 32],
}

#[program]
pub mod nft_listing_v2 {
    use super::*;

    /// `listing_account` の owner チェックを行っていないため、
    /// 攻撃者が任意アカウントを指定して他人のNFTを無断で出品できます
    pub fn list_nft(
        ctx: Context<ListNft>,
        price: u64,        // 出品価格 (lamports)
        fee_bps: u16,      // 手数料率 (bps)
        expiry: u64,       // 出品有効期限（slot）
    ) -> Result<()> {
        let acct = &mut ctx.accounts.listing_account.to_account_info();
        let data = &mut acct.data.borrow_mut();

        // 構造体全体のバイト長を取得
        let size = std::mem::size_of::<ListingData>();
        if data.len() < size {
            return err!(ErrorCode::DataTooShort);
        }

        // 値を埋めた ListingData を作成
        let mut listing = ListingData::zeroed();
        listing.price   = price;
        listing.fee_bps = fee_bps;
        listing.expiry  = expiry;
        listing.seller  = ctx.accounts.seller.key().to_bytes();

        // メモリ全体を一括コピー（slice は範囲指定のみ）
        let bytes = bytemuck::bytes_of(&listing);
        data[..size].copy_from_slice(bytes);

        msg!(
            "NFT {} listed by {} → price={}, fee={}bps, expires at slot={}",
            acct.key(),
            ctx.accounts.seller.key(),
            price,
            fee_bps,
            expiry
        );
        Ok(())
    }
}

#[derive(Accounts)]
pub struct ListNft<'info> {
    /// CHECK: owner == program_id の検証を一切行っていない AccountInfo<'info>
    #[account(mut)]
    pub listing_account: AccountInfo<'info>,

    /// 出品者署名のみを検証
    pub seller: Signer<'info>,
}

#[error_code]
pub enum ErrorCode {
    #[msg("アカウントデータ長が想定より短いです")]
    DataTooShort,
}
