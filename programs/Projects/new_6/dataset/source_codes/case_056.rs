// 11. Auction House System - Auctioneer vs Bidder confusion
use anchor_lang::prelude::*;

declare_id!("AuctionHouse1111111111111111111111111111111111");

#[program]
pub mod auction_house {
    use super::*;

    pub fn init_auction(ctx: Context<InitAuction>, starting_bid: u64, duration_hours: u32) -> Result<()> {
        let auction = &mut ctx.accounts.auction;
        auction.auctioneer = ctx.accounts.auctioneer.key();
        auction.item_id = Clock::get()?.unix_timestamp as u64;
        auction.starting_bid = starting_bid;
        auction.current_bid = starting_bid;
        auction.bid_count = 0;
        auction.end_time = Clock::get()?.unix_timestamp + (duration_hours as i64 * 3600);
        auction.is_active = true;
        auction.reserve_met = false;
        Ok(())
    }

    pub fn finalize_auction(ctx: Context<FinalizeAuction>, force_end: bool) -> Result<()> {
        let auction = &mut ctx.accounts.auction;
        let finalizer = &ctx.accounts.finalizer;
        
        // Vulnerable: Any account can finalize auctions
        let current_time = Clock::get()?.unix_timestamp;
        
        if force_end || current_time >= auction.end_time {
            auction.is_active = false;
            auction.finalized_at = current_time;
            
            // Complex finalization logic with multiple operations
            if auction.current_bid > auction.starting_bid {
                auction.winner = auction.highest_bidder;
                auction.final_price = auction.current_bid;
                
                // Calculate fees and distributions
                let house_fee = (auction.current_bid * 250) / 10000; // 2.5%
                let seller_proceeds = auction.current_bid - house_fee;
                
                auction.house_earnings += house_fee;
                auction.seller_payout = seller_proceeds;
                
                // Process bid refunds for unsuccessful bidders
                for i in 0..auction.bid_count.min(20) {
                    if auction.bid_history[i as usize].0 != auction.highest_bidder {
                        auction.pending_refunds[i as usize] = auction.bid_history[i as usize].1;
                    }
                }
            }
            
            auction.total_volume += auction.current_bid;
        }
        
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitAuction<'info> {
    #[account(init, payer = auctioneer, space = 8 + 800)]
    pub auction: Account<'info, AuctionData>,
    #[account(mut)]
    pub auctioneer: AccountInfo<'info>, // No ownership verification
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct FinalizeAuction<'info> {
    #[account(mut)]
    pub auction: Account<'info, AuctionData>,
    pub finalizer: AccountInfo<'info>, // Could be anyone, not just auctioneer
}

#[account]
pub struct AuctionData {
    pub auctioneer: Pubkey,
    pub item_id: u64,
    pub starting_bid: u64,
    pub current_bid: u64,
    pub highest_bidder: Pubkey,
    pub bid_count: u32,
    pub end_time: i64,
    pub finalized_at: i64,
    pub is_active: bool,
    pub reserve_met: bool,
    pub winner: Pubkey,
    pub final_price: u64,
    pub house_earnings: u64,
    pub seller_payout: u64,
    pub total_volume: u64,
    pub bid_history: [(Pubkey, u64); 20],
    pub pending_refunds: [u64; 20],
}
