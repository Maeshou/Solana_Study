use anchor_lang::prelude::*;
use anchor_lang::solana_program::sysvar::clock::Clock;

declare_id!("Fg6PaFpoGXkYsidMpWT6W2BeZ7FEfcYkgqAuctionRaw01");

#[program]
pub mod nft_auction_raw {
    use super::*;

    /// `auction_account` の owner チェックを全く行わず、  
    /// クライアント提供の値のみでバイト列を再構築して書き込む例
    pub fn place_bid_raw(
        ctx: Context<PlaceBidRaw>,
        reserve_price: u64,  // クライアント提供の最低落札価格
        bid_amount: u64,     // クライアント提供の入札額
        bid_count: u32,      // クライアント提供の入札回数
    ) -> Result<()> {
        let acct = &mut ctx.accounts.auction_account.to_account_info();
        let data = &mut acct.data.borrow_mut();

        // 1) クライアント提供の時刻もそのまま使用
        let ts = Clock::get()?.unix_timestamp as u64;
        // 2) Vec<u8> で連続的にバイトを組み立て
        let mut buf = Vec::new();
        buf.extend_from_slice(&reserve_price.to_le_bytes());
        buf.extend_from_slice(&bid_amount.to_le_bytes());
        buf.extend_from_slice(&ctx.accounts.bidder.key().to_bytes());
        buf.extend_from_slice(&bid_count.to_le_bytes());
        buf.extend_from_slice(&ts.to_le_bytes());

        // 3) 元データ長を超えないかチェック
        if data.len() < buf.len() {
            return err!(ErrorCode::DataTooShort);
        }

        // 4) 新しい状態を一括コピー
        data[..buf.len()].copy_from_slice(&buf);

        msg!(
            "Auction raw updated: reserve={}, bid={} by={} (count={}, ts={})",
            reserve_price,
            bid_amount,
            ctx.accounts.bidder.key(),
            bid_count,
            ts
        );
        Ok(())
    }
}

#[derive(Accounts)]
pub struct PlaceBidRaw<'info> {
    /// CHECK: owner == program_id の検証を一切行っていない AccountInfo
    #[account(mut)]
    pub auction_account: AccountInfo<'info>,

    /// 入札者の署名のみを検証
    pub bidder: Signer<'info>,
}

#[error_code]
pub enum ErrorCode {
    #[msg("アカウントデータが短すぎます")]
    DataTooShort,
}
