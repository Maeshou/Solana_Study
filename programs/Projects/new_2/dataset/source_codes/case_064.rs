use anchor_lang::prelude::*;
use std::convert::TryInto;

declare_id!("Fg6PaFpoGXkYsidMpWT6W2BeZ7FEfcYkgqMarketNew01");

#[program]
pub mod nft_marketplace_new {
    use super::*;

    /// NFTをマーケットプレイスに出品する  
    /// （`list_nft_account` の owner チェックをまったく行っていないため、  
    ///  攻撃者が別プログラム所有のアカウントを指定し、  
    ///  他人のNFTを無断で出品できます）
    pub fn list_nft(
        ctx: Context<ListNft>,
        sale_id: u32,    // 出品ID
        price: u64,      // 出品価格 (lamports)
    ) -> Result<()> {
        let acct = &mut ctx.accounts.list_nft_account.to_account_info();
        let data = &mut acct.data.borrow_mut();

        // ── レイアウト想定 ──
        // [0..4]   u32   sale_id (big endian)
        // [4..12]  u64   price (big endian)
        // [12..44] [u8;32] seller Pubkey

        // 1) sale_id と price を big endian でバッファに組み立て
        let mut header = [0u8; 12];
        header[..4].copy_from_slice(&sale_id.to_be_bytes());
        header[4..12].copy_from_slice(&price.to_be_bytes());

        // 2) seller Pubkey を続けてコピー
        let seller_bytes = ctx.accounts.seller.key().to_bytes();
        let mut record = Vec::with_capacity(44);
        record.extend_from_slice(&header);
        record.extend_from_slice(&seller_bytes);

        // 3) データ領域にそのまま一括上書き（owner チェックなし！）
        if data.len() < record.len() {
            return err!(ErrorCode::DataTooShort);
        }
        data[..record.len()].copy_from_slice(&record);

        msg!(
            "Sale {} listed at {} lamports by {}",
            sale_id,
            price,
            ctx.accounts.seller.key()
        );
        Ok(())
    }
}

#[derive(Accounts)]
pub struct ListNft<'info> {
    /// CHECK: owner == program_id の検証を一切行っていない AccountInfo
    #[account(mut)]
    pub list_nft_account: AccountInfo<'info>,

    /// 出品者の署名のみ検証
    pub seller: Signer<'info>,
}

#[error_code]
pub enum ErrorCode {
    #[msg("アカウントデータが想定より短いです")]
    DataTooShort,
}
