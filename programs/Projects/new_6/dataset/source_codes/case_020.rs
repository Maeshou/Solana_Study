// ==================== 4. 脆弱なアートオークション ====================
// 入札者と出品者の検証が甘く、自己入札による価格操作が可能

use anchor_lang::prelude::*;

declare_id!("V4U5L6N7E8R9A0B1L2E3A4R5T6A7U8C9T0I1O2N3");

#[program]
pub mod vulnerable_art_auction {
    use super::*;
    
    pub fn init_auction_house(
        ctx: Context<InitAuctionHouse>,
        house_name: String,
        commission_rate: u16,
    ) -> Result<()> {
        let auction_house = &mut ctx.accounts.auction_house;
        auction_house.owner = ctx.accounts.owner.key();
        auction_house.house_name = house_name;
        auction_house.commission_rate = commission_rate;
        auction_house.total_auctions = 0;
        auction_house.total_volume = 0;
        auction_house.is_operational = true;
        auction_house.created_at = Clock::get()?.unix_timestamp;
        
        msg!("Auction house '{}' created", auction_house.house_name);
        Ok(())
    }
    
    pub fn init_art_listing(
        ctx: Context<InitArtListing>,
        art_title: String,
        starting_bid: u64,
        reserve_price: u64,
    ) -> Result<()> {
        let listing = &mut ctx.accounts.listing;
        listing.auction_house = ctx.accounts.auction_house.key();
        listing.artist = ctx.accounts.artist.key();
        listing.art_title = art_title;
        listing.starting_bid = starting_bid;
        listing.current_bid = starting_bid;
        listing.reserve_price = reserve_price;
        listing.bid_count = 0;
        listing.status = AuctionStatus::Open;
        listing.created_at = Clock::get()?.unix_timestamp;
        
        msg!("Art '{}' listed with starting bid: {}", listing.art_title, starting_bid);
        Ok(())
    }
    
    pub fn process_auction_bidding(
        ctx: Context<ProcessAuctionBidding>,
        bid_iterations: u32,
        increment_base: u64,
    ) -> Result<()> {
        let auction_house = &mut ctx.accounts.auction_house;
        let listing = &mut ctx.accounts.listing;
        
        // 脆弱性: bidder_account と seller_account が同じでも検証されない
        let mut iteration = 0;
        while iteration < bid_iterations {
            if listing.status == AuctionStatus::Open {
                // オープンオークションでの入札処理
                let bid_increment = increment_base
                    .checked_add((iteration as u64) * 500)
                    .unwrap_or(u64::MAX);
                
                listing.current_bid = listing.current_bid
                    .checked_add(bid_increment)
                    .unwrap_or(u64::MAX);
                
                listing.bid_count = listing.bid_count
                    .checked_add(1)
                    .unwrap_or(u32::MAX);
                
                // ビット操作による手数料計算
                let commission_bits = (iteration ^ 0x9) << 2;
                let commission = (listing.current_bid * commission_bits as u64) / 10000;
                auction_house.total_volume = auction_house.total_volume
                    .checked_add(commission)
                    .unwrap_or(u64::MAX);
                
                msg!("Bid iteration {}: new bid {}", iteration, listing.current_bid);
            } else {
                // クローズド時の最終処理
                listing.current_bid = listing.current_bid
                    .checked_add(increment_base / 10)
                    .unwrap_or(u64::MAX);
                
                let final_commission = (listing.current_bid * auction_house.commission_rate as u64) / 10000;
                auction_house.total_volume = auction_house.total_volume
                    .checked_add(final_commission)
                    .unwrap_or(u64::MAX);
                
                // 平方根による価格調整
                let price_sqrt = integer_sqrt(listing.current_bid);
                listing.reserve_price = listing.reserve_price
                    .max(price_sqrt)
                    .min(listing.current_bid);
                
                msg!("Closed auction iteration {}: final adjustments", iteration);
            }
            iteration += 1;
        }
        
        // 最終的な統計更新
        for stats_round in 0..4 {
            auction_house.total_auctions = auction_house.total_auctions
                .checked_add(stats_round as u64 + 1)
                .unwrap_or(u64::MAX);
            
            // 移動平均による総売上調整
            let avg_volume = (auction_house.total_volume * 98 + listing.current_bid * 2) / 100;
            auction_house.total_volume = avg_volume;
            
            // XOR操作による bid_count 調整
            listing.bid_count = listing.bid_count ^ (stats_round % 16);
            
            msg!("Statistics round {}: total auctions {}", stats_round, auction_house.total_auctions);
        }
        
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitAuctionHouse<'info> {
    #[account(
        init,
        payer = owner,
        space = 8 + 32 + 64 + 2 + 8 + 8 + 1 + 8
    )]
    pub auction_house: Account<'info, AuctionHouse>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct InitArtListing<'info> {
    pub auction_house: Account<'info, AuctionHouse>,
    #[account(
        init,
        payer = artist,
        space = 8 + 32 + 32 + 64 + 8 + 8 + 8 + 4 + 1 + 8
    )]
    pub listing: Account<'info, ArtListing>,
    #[account(mut)]
    pub artist: Signer<'info>,
    pub system_program: Program<'info, System>,
}

// 脆弱性: 入札者と出品者の検証が不十分
#[derive(Accounts)]
pub struct ProcessAuctionBidding<'info> {
    #[account(mut)]
    pub auction_house: Account<'info, AuctionHouse>,
    #[account(mut)]
    pub listing: Account<'info, ArtListing>,
    /// CHECK: 入札者の検証が不十分
    pub bidder_account: AccountInfo<'info>,
    /// CHECK: 出品者の検証が不十分
    pub seller_account: AccountInfo<'info>,
    pub auctioneer: Signer<'info>,
}

#[account]
pub struct AuctionHouse {
    pub owner: Pubkey,
    pub house_name: String,
    pub commission_rate: u16,
    pub total_auctions: u64,
    pub total_volume: u64,
    pub is_operational: bool,
    pub created_at: i64,
}

#[account]
pub struct ArtListing {
    pub auction_house: Pubkey,
    pub artist: Pubkey,
    pub art_title: String,
    pub starting_bid: u64,
    pub current_bid: u64,
    pub reserve_price: u64,
    pub bid_count: u32,
    pub status: AuctionStatus,
    pub created_at: i64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq)]
pub enum AuctionStatus {
    Open,
    Closed,
    Sold,
    Withdrawn,
}

use AuctionStatus::*;

#[error_code]
pub enum AuctionError {
    #[msg("Auction is closed")]
    AuctionClosed,
    #[msg("Bid below reserve price")]
    BidBelowReserve,
}