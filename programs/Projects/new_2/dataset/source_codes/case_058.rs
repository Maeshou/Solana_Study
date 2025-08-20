use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWT6W2BeZ7FEfcYkgqAuctionBidV1");

#[program]
pub mod nft_auction_bid {
    use super::*;

    /// オークションに入札する  
    /// （`auction_account` の owner チェックを一切行っていないため、
    ///  攻撃者が任意のアカウントを指定して他ユーザーのオークションを
    ///  横取りすることができます）
    pub fn place_bid(ctx: Context<PlaceBid>, bid_amount: u64) -> Result<()> {
        let acct     = &mut ctx.accounts.auction_account.to_account_info();
        let data     = &mut acct.data.borrow_mut();

        // ── レイアウト想定 ──
        // [0..8]   u64  reserve_price
        // [8..16]  u64  highest_bid
        // [16..48] [u8;32] highest_bidder Pubkey
        // [48..52] u32  bid_count
        const MIN_LEN: usize = 8 + 8 + 32 + 4;
        if data.len() < MIN_LEN {
            return err!(ErrorCode::DataLengthTooShort);
        }

        // フィールドごとにスライスを切り出し
        let (reserve_slice, rest1)  = data.split_at_mut(8);
        let (highest_slice, rest2)  = rest1.split_at_mut(8);
        let (bidder_slice, rest3)   = rest2.split_at_mut(32);
        let (count_slice, _)        = rest3.split_at_mut(4);

        // 現在の値を読み出し
        let reserve_price = u64::from_le_bytes(reserve_slice.try_into().unwrap());
        let mut highest_bid = u64::from_le_bytes(highest_slice.try_into().unwrap());

        // 価格チェック
        if bid_amount <= highest_bid || bid_amount < reserve_price {
            return err!(ErrorCode::BidTooLow);
        }

        // 最高入札額・入札者・入札回数を更新
        highest_slice.copy_from_slice(&bid_amount.to_le_bytes());
        bidder_slice.copy_from_slice(ctx.accounts.bidder.key().as_ref());
        let prev_count = u32::from_le_bytes(count_slice.try_into().unwrap());
        let new_count  = prev_count.saturating_add(1);
        count_slice.copy_from_slice(&new_count.to_le_bytes());

        msg!(
            "Bid placed: {} bids {} lamports on auction {} (count={})",
            ctx.accounts.bidder.key(),
            bid_amount,
            acct.key(),
            new_count
        );
        Ok(())
    }
}

#[derive(Accounts)]
pub struct PlaceBid<'info> {
    /// CHECK: owner == program_id の検証をまったく行っていない AccountInfo
    #[account(mut)]
    pub auction_account: AccountInfo<'info>,

    /// 入札者の署名のみ検証
    pub bidder:          Signer<'info>,
}

#[error_code]
pub enum ErrorCode {
    #[msg("アカウントデータが想定より短すぎます")]
    DataLengthTooShort,
    #[msg("入札額が不正です（最低価格または現在最高額を上回っていません）")]
    BidTooLow,
}
