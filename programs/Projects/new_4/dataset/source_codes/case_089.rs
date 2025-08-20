use anchor_lang::prelude::*;

declare_id!("Repertory18Oracle1111111111111111111111111111");

#[program]
pub mod price_oracle {
    use super::*;

    // オラクルフィードを初期化
    pub fn init_feed(ctx: Context<InitFeed>) -> Result<()> {
        let f = &mut ctx.accounts.feed;
        f.samples = Vec::new();
        Ok(())
    }

    // 平均価格を計算
    pub fn update_feed(ctx: Context<UpdateFeed>, price_points: Vec<u64>) -> Result<()> {
        let f = &mut ctx.accounts.feed;            // ← initなし：既存参照
        let mut sum = 0u128;
        for &p in price_points.iter() {
            sum += p as u128;
        }
        let avg = if price_points.is_empty() { 0 } else { (sum / price_points.len() as u128) as u64 };
        f.last_price = avg;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitFeed<'info> {
    #[account(init, payer = oracle, space = 8 + 8 + 4 + 8)]
    pub feed: Account<'info, FeedData>,
    #[account(mut)] pub oracle: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateFeed<'info> {
    pub feed: Account<'info, FeedData>,
    #[account(mut)] pub oracle: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct FeedData {
    pub last_price: u64,
    pub samples: Vec<u64>,
}
