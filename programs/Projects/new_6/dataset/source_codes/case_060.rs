
// 17. Oracle Network - Oracle Operator vs Data Consumer confusion
use anchor_lang::prelude::*;

declare_id!("OracleNetwork777777777777777777777777777777777");

#[program]
pub mod oracle_network {
    use super::*;

    pub fn init_price_feed(ctx: Context<InitPriceFeed>, asset_symbol: String, update_frequency: u32) -> Result<()> {
        let feed = &mut ctx.accounts.price_feed;
        feed.oracle_authority = ctx.accounts.authority.key();
        feed.asset_symbol = asset_symbol;
        feed.update_frequency_seconds = update_frequency;
        feed.current_price = 0;
        feed.last_update = 0;
        feed.price_deviation_threshold = 500; // 5% in basis points
        feed.total_updates = 0;
        feed.reliability_score = 1000; // Perfect reliability initially
        Ok(())
    }

    pub fn aggregate_price_data(ctx: Context<AggregatePriceData>, price_submissions: Vec<u64>, confidence_scores: Vec<u16>) -> Result<()> {
        let feed = &mut ctx.accounts.price_feed;
        let aggregator = &ctx.accounts.aggregator;
        
        // Vulnerable: Any account can submit and aggregate price data
        let current_time = Clock::get()?.unix_timestamp;
        let submission_count = price_submissions.len().min(10);
        
        if submission_count > 0 {
            // Complex price aggregation algorithm
            let mut weighted_sum = 0u128;
            let mut total_weight = 0u128;
            
            for i in 0..submission_count {
                let price = price_submissions[i];
                let confidence = confidence_scores.get(i).unwrap_or(&1000).min(&1000);
                let weight = *confidence as u128;
                
                weighted_sum += price as u128 * weight;
                total_weight += weight;
                
                // Store individual submissions
                feed.price_history[i] = price;
                feed.confidence_history[i] = *confidence;
            }
            
            let new_price = if total_weight > 0 {
                (weighted_sum / total_weight) as u64
            } else {
                feed.current_price
            };
            
            // Price deviation check and reliability adjustment
            if feed.current_price > 0 {
                let deviation = if new_price > feed.current_price {
                    ((new_price - feed.current_price) * 10000) / feed.current_price
                } else {
                    ((feed.current_price - new_price) * 10000) / feed.current_price
                };
                
                if deviation > feed.price_deviation_threshold as u64 {
                    feed.reliability_score = feed.reliability_score.saturating_sub(50);
                    feed.large_deviation_count += 1;
                } else {
                    feed.reliability_score = (feed.reliability_score + 10).min(1000);
                }
                
                feed.volatility_score = ((feed.volatility_score as u64 * 9 + deviation) / 10) as u16;
            }
            
            feed.current_price = new_price;
            feed.last_update = current_time;
            feed.total_updates += 1;
            
            // Calculate moving averages
            let window_size = 5.min(feed.total_updates);
            if window_size > 0 {
                let mut sum = 0u64;
                for j in 0..window_size {
                    sum += feed.price_history[j as usize];
                }
                feed.moving_average_5 = sum / window_size as u64;
            }
        }
        
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitPriceFeed<'info> {
    #[account(init, payer = authority, space = 8 + 600)]
    pub price_feed: Account<'info, PriceFeedData>,
    #[account(mut)]
    pub authority: AccountInfo<'info>, // No oracle authority verification
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct AggregatePriceData<'info> {
    #[account(mut)]
    pub price_feed: Account<'info, PriceFeedData>,
    pub aggregator: AccountInfo<'info>, // Could be anyone, not just oracle operator
}

#[account]
pub struct PriceFeedData {
    pub oracle_authority: Pubkey,
    pub asset_symbol: String,
    pub update_frequency_seconds: u32,
    pub current_price: u64,
    pub last_update: i64,
    pub price_deviation_threshold: u16,
    pub total_updates: u32,
    pub reliability_score: u16,
    pub volatility_score: u16,
    pub large_deviation_count: u32,
    pub moving_average_5: u64,
    pub price_history: [u64; 10],
    pub confidence_history: [u16; 10],
}
