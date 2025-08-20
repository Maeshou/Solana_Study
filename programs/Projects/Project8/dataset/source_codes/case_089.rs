// 7) bidding_bootstrap: 初期入札履歴・リーダー・集計・リザーブ達成
use anchor_lang::prelude::*;

declare_id!("BidBoot44444444444444444444444444444444");

#[program]
pub mod bidding_bootstrap {
    use super::*;

    pub fn seed_bidding(
        ctx: Context<SeedBidding>,
        seller: Pubkey,
        starting_price: u64,
        reserve_price: u64,
    ) -> Result<()> {
        let now = Clock::get()?.unix_timestamp;

        let mut history: Vec<BidRecord> = Vec::new();
        history.push(BidRecord {
            bidder: seller,
            bid_amount: starting_price,
            bid_timestamp: now,
            bid_type: BidType::StartingPrice,
            automatic_bid: false,
        });

        let reserve_met = starting_price >= reserve_price;

        let book = &mut ctx.accounts.bids;
        book.bidding_history = history;
        book.total_bids_placed = 0;
        book.unique_bidders = 0;
        book.current_leader = None;
        book.reserve_price_met = reserve_met;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct SeedBidding<'info> {
    #[account(
        init,
        payer = creator,
        space = 8 + BidBook::LEN,
        seeds = [b"bids", creator.key().as_ref()],
        bump
    )]
    pub bids: Account<'info, BidBook>,
    #[account(mut)]
    pub creator: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct BidBook {
    pub bidding_history: Vec<BidRecord>,
    pub total_bids_placed: u32,
    pub unique_bidders: u32,
    pub current_leader: Option<Pubkey>,
    pub reserve_price_met: bool,
}
impl BidBook { pub const LEN: usize = 4 + 32 * BidRecord::LEN + 4 + 4 + 33 + 1; }

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct BidRecord {
    pub bidder: Pubkey,
    pub bid_amount: u64,
    pub bid_timestamp: i64,
    pub bid_type: BidType,
    pub automatic_bid: bool,
}
impl BidRecord { pub const LEN: usize = 32 + 8 + 8 + 1 + 1; }

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy)]
pub enum BidType { StartingPrice, RegularBid, AutoBid, BuyoutPurchase }
