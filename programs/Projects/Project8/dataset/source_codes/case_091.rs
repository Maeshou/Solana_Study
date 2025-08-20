// 9) analytics_initializer: 市場分析の初期値（分岐少なめ）
use anchor_lang::prelude::*;

declare_id!("AnalInit444444444444444444444444444444");

#[program]
pub mod analytics_initializer {
    use super::*;

    pub fn init_analytics(ctx: Context<InitAnalytics>, category_avg_price: u64, demand_hint: u8) -> Result<()> {
        let mut demand = DemandLevel::Moderate;
        if demand_hint >= 3 { demand = DemandLevel::High; }
        if demand_hint >= 4 { demand = DemandLevel::VeryHigh; }
        if demand_hint == 0 { demand = DemandLevel::Low; }

        ctx.accounts.analytics.market_analytics = MarketAnalytics {
            category_average_price: category_avg_price,
            recent_sales_comparison: Vec::new(),
            price_trend_indicator: PriceTrend::Stable,
            demand_level: demand,
            supply_scarcity_rating: 5,
        };
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitAnalytics<'info> {
    #[account(
        init,
        payer = reporter,
        space = 8 + AnalyticsSlot::LEN,
        seeds = [b"analytics", reporter.key().as_ref()],
        bump
    )]
    pub analytics: Account<'info, AnalyticsSlot>,
    #[account(mut)]
    pub reporter: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct AnalyticsSlot { pub market_analytics: MarketAnalytics }
impl AnalyticsSlot { pub const LEN: usize = MarketAnalytics::LEN; }

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct MarketAnalytics {
    pub category_average_price: u64,
    pub recent_sales_comparison: Vec<SaleComparison>,
    pub price_trend_indicator: PriceTrend,
    pub demand_level: DemandLevel,
    pub supply_scarcity_rating: u32,
}
impl MarketAnalytics { pub const LEN: usize = 8 + (4 + 16 * SaleComparison::LEN) + 1 + 1 + 4; }

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct SaleComparison {
    pub sold_price: u64,
    pub sale_date: i64,
    pub item_similarity_score: u32,
}
impl SaleComparison { pub const LEN: usize = 8 + 8 + 4; }

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy)]
pub enum PriceTrend { Rising, Stable, Declining }

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy)]
pub enum DemandLevel { Low, Moderate, High, VeryHigh }
