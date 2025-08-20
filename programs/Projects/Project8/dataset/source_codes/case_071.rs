use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount, Mint};

declare_id!("44444444444444444444444444444444");

#[program]
pub mod marketplace_auction_system {
    use super::*;

    pub fn create_auction_listing(
        ctx: Context<CreateAuctionListing>,
        item_details: ItemDetails,
        starting_price: u64,
        auction_duration: u64,
        buyout_price: Option<u64>,
    ) -> Result<()> {
        let auction_listing = &mut ctx.accounts.auction_listing;
        let current_time = Clock::get()?.unix_timestamp;
        
        auction_listing.seller = ctx.accounts.seller.key();
        auction_listing.item_being_sold = item_details;
        auction_listing.starting_price = starting_price;
        auction_listing.current_highest_bid = starting_price;
        auction_listing.buyout_price = buyout_price;
        auction_listing.auction_start_time = current_time;
        auction_listing.auction_end_time = current_time + auction_duration as i64;
        auction_listing.listing_creation_timestamp = current_time;
        
        require!(starting_price >= 1000, AuctionError::StartingPriceTooLow);
        require!(auction_duration >= 3600, AuctionError::AuctionTooShort);
        require!(auction_duration <= 604800, AuctionError::AuctionTooLong);
        
        let category_multiplier = match item_details.category {
            ItemCategory::LegendaryWeapon => {
                auction_listing.minimum_bid_increment = starting_price / 20; // 5% increment
                auction_listing.featured_listing = true;
                auction_listing.seller_reputation_requirement = 100;
                300
            },
            ItemCategory::RareArmor => {
                auction_listing.minimum_bid_increment = starting_price / 25; // 4% increment
                auction_listing.featured_listing = true;
                auction_listing.seller_reputation_requirement = 75;
                200
            },
            ItemCategory::EpicAccessory => {
                auction_listing.minimum_bid_increment = starting_price / 30; // ~3.3% increment
                auction_listing.featured_listing = false;
                auction_listing.seller_reputation_requirement = 50;
                250
            },
            ItemCategory::CraftingMaterial => {
                auction_listing.minimum_bid_increment = starting_price / 50; // 2% increment
                auction_listing.featured_listing = false;
                auction_listing.seller_reputation_requirement = 25;
                100
            },
            ItemCategory::ConsumableItem => {
                auction_listing.minimum_bid_increment = starting_price / 100; // 1% increment
                auction_listing.featured_listing = false;
                auction_listing.seller_reputation_requirement = 10;
                50
            },
        };
        
        auction_listing.estimated_final_price = starting_price + (starting_price * category_multiplier / 100);
        
        let mut bidding_history = Vec::new();
        bidding_history.push(BidRecord {
            bidder: ctx.accounts.seller.key(), // Initial "bid" by seller
            bid_amount: starting_price,
            bid_timestamp: current_time,
            bid_type: BidType::StartingPrice,
            automatic_bid: false,
        });
        
        auction_listing.bidding_history = bidding_history;
        auction_listing.total_bids_placed = 0;
        auction_listing.unique_bidders = 0;
        auction_listing.current_leader = None;
        auction_listing.reserve_price_met = starting_price >= item_details.reserve_price;
        
        let mut watchers_list = Vec::new();
        for watcher_slot in 0..20 { // Pre-allocate 20 watcher slots
            watchers_list.push(WatcherInfo {
                watcher_pubkey: None,
                watch_start_time: 0,
                notification_preferences: NotificationPreferences {
                    bid_outbid_alert: false,
                    auction_ending_alert: false,
                    price_threshold_alert: false,
                    threshold_amount: 0,
                },
            });
        }
        auction_listing.watchers = watchers_list;
        
        let fee_structure = match item_details.estimated_value {
            value if value >= 100000 => {
                auction_listing.listing_fee = 500;
                auction_listing.success_fee_percentage = 250; // 2.5%
                auction_listing.premium_features_enabled = true;
                FeeStructure::Premium
            },
            value if value >= 50000 => {
                auction_listing.listing_fee = 300;
                auction_listing.success_fee_percentage = 300; // 3.0%
                auction_listing.premium_features_enabled = true;
                FeeStructure::Standard
            },
            value if value >= 10000 => {
                auction_listing.listing_fee = 150;
                auction_listing.success_fee_percentage = 400; // 4.0%
                auction_listing.premium_features_enabled = false;
                FeeStructure::Basic
            },
            _ => {
                auction_listing.listing_fee = 50;
                auction_listing.success_fee_percentage = 500; // 5.0%
                auction_listing.premium_features_enabled = false;
                FeeStructure::Economy
            },
        };
        
        auction_listing.fee_structure = fee_structure;
        
        let mut promotional_boosts = Vec::new();
        match auction_listing.premium_features_enabled {
            true => {
                promotional_boosts.push(PromotionalBoost {
                    boost_type: BoostType::FeaturedPlacement,
                    duration_hours: 24,
                    cost_paid: 200,
                    activation_time: current_time,
                });
                promotional_boosts.push(PromotionalBoost {
                    boost_type: BoostType::HighlightListing,
                    duration_hours: 48,
                    cost_paid: 150,
                    activation_time: current_time,
                });
                promotional_boosts.push(PromotionalBoost {
                    boost_type: BoostType::CrossCategoryPromotion,
                    duration_hours: 72,
                    cost_paid: 300,
                    activation_time: current_time,
                });
            },
            false => {
                promotional_boosts.push(PromotionalBoost {
                    boost_type: BoostType::BasicVisibility,
                    duration_hours: 12,
                    cost_paid: 50,
                    activation_time: current_time,
                });
            }
        }
        auction_listing.promotional_boosts = promotional_boosts;
        
        auction_listing.auction_status = AuctionStatus::Active;
        auction_listing.auto_extension_enabled = true;
        auction_listing.extension_time_minutes = 300; // 5 minutes
        auction_listing.snipe_protection_active = true;
        
        auction_listing.market_analytics = MarketAnalytics {
            category_average_price: item_details.estimated_value,
            recent_sales_comparison: Vec::new(),
            price_trend_indicator: PriceTrend::Stable,
            demand_level: DemandLevel::Moderate,
            supply_scarcity_rating: 5,
        };
        
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreateAuctionListing<'info> {
    #[account(
        init,
        payer = seller,
        space = 8 + AuctionListing::LEN,
        seeds = [b"auction", seller.key().as_ref()],
        bump
    )]
    pub auction_listing: Account<'info, AuctionListing>,
    
    #[account(mut)]
    pub seller: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}

#[account]
pub struct AuctionListing {
    pub seller: Pubkey,
    pub item_being_sold: ItemDetails,
    pub starting_price: u64,
    pub current_highest_bid: u64,
    pub buyout_price: Option<u64>,
    pub auction_start_time: i64,
    pub auction_end_time: i64,
    pub listing_creation_timestamp: i64,
    pub minimum_bid_increment: u64,
    pub featured_listing: bool,
    pub seller_reputation_requirement: u32,
    pub estimated_final_price: u64,
    pub bidding_history: Vec<BidRecord>,
    pub total_bids_placed: u32,
    pub unique_bidders: u32,
    pub current_leader: Option<Pubkey>,
    pub reserve_price_met: bool,
    pub watchers: Vec<WatcherInfo>,
    pub listing_fee: u64,
    pub success_fee_percentage: u32,
    pub premium_features_enabled: bool,
    pub fee_structure: FeeStructure,
    pub promotional_boosts: Vec<PromotionalBoost>,
    pub auction_status: AuctionStatus,
    pub auto_extension_enabled: bool,
    pub extension_time_minutes: u32,
    pub snipe_protection_active: bool,
    pub market_analytics: MarketAnalytics,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct ItemDetails {
    pub item_name: String,
    pub description: String,
    pub category: ItemCategory,
    pub rarity_level: RarityLevel,
    pub condition_rating: u32,
    pub enhancement_level: u32,
    pub estimated_value: u64,
    pub reserve_price: u64,
    pub authentication_verified: bool,
    pub previous_owners: Vec<Pubkey>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub enum ItemCategory {
    LegendaryWeapon,
    RareArmor,
    EpicAccessory,
    CraftingMaterial,
    ConsumableItem,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub enum RarityLevel {
    Common,
    Uncommon,
    Rare,
    Epic,
    Legendary,
    Mythical,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct BidRecord {
    pub bidder: Pubkey,
    pub bid_amount: u64,
    pub bid_timestamp: i64,
    pub bid_type: BidType,
    pub automatic_bid: bool,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub enum BidType {
    StartingPrice,
    RegularBid,
    AutoBid,
    BuyoutPurchase,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct WatcherInfo {
    pub watcher_pubkey: Option<Pubkey>,
    pub watch_start_time: i64,
    pub notification_preferences: NotificationPreferences,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct NotificationPreferences {
    pub bid_outbid_alert: bool,
    pub auction_ending_alert: bool,
    pub price_threshold_alert: bool,
    pub threshold_amount: u64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub enum FeeStructure {
    Economy,
    Basic,
    Standard,
    Premium,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct PromotionalBoost {
    pub boost_type: BoostType,
    pub duration_hours: u32,
    pub cost_paid: u64,
    pub activation_time: i64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub enum BoostType {
    BasicVisibility,
    FeaturedPlacement,
    HighlightListing,
    CrossCategoryPromotion,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub enum AuctionStatus {
    Active,
    Ended,
    Cancelled,
    Suspended,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct MarketAnalytics {
    pub category_average_price: u64,
    pub recent_sales_comparison: Vec<SaleComparison>,
    pub price_trend_indicator: PriceTrend,
    pub demand_level: DemandLevel,
    pub supply_scarcity_rating: u32,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct SaleComparison {
    pub sold_price: u64,
    pub sale_date: i64,
    pub item_similarity_score: u32,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub enum PriceTrend {
    Rising,
    Stable,
    Declining,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub enum DemandLevel {
    Low,
    Moderate,
    High,
    VeryHigh,
}

impl AuctionListing {
    pub const LEN: usize = 32 + 300 + 8 + 8 + 9 + 8 + 8 + 8 + 8 + 1 + 4 + 8 + 800 + 4 + 4 + 33 + 1 + 600 + 8 + 4 + 1 + 1 + 300 + 1 + 1 + 4 + 1 + 200;
}

#[error_code]
pub enum AuctionError {
    #[msg("Starting price is too low")]
    StartingPriceTooLow,
    #[msg("Auction duration is too short")]
    AuctionTooShort,
    #[msg("Auction duration exceeds maximum allowed")]
    AuctionTooLong,
}