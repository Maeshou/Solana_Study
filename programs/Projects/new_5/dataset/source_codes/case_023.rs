// 4. Token & Item Exchange
declare_id!("H6J9K2L6M0N4P8Q1R5S9T3U7V1W5X9Y3Z7A1B5");

use anchor_lang::prelude::*;

#[program]
pub mod item_exchange_insecure {
    use super::*;

    pub fn init_market(ctx: Context<InitMarket>, name: String) -> Result<()> {
        let market = &mut ctx.accounts.market;
        market.owner = ctx.accounts.owner.key();
        market.name = name;
        market.is_active = true;
        market.trade_count = 0;
        msg!("Market '{}' initialized.", market.name);
        Ok(())
    }

    pub fn init_offer(ctx: Context<InitOffer>, offer_id: u64, price: u64) -> Result<()> {
        let offer = &mut ctx.accounts.offer;
        let market = &mut ctx.accounts.market;

        offer.market = market.key();
        offer.offer_id = offer_id;
        offer.lister = ctx.accounts.lister.key();
        offer.price = price;
        offer.is_fulfilled = false;
        
        market.trade_count = market.trade_count.saturating_add(1);
        msg!("Offer {} created for market {}.", offer.offer_id, market.name);
        Ok(())
    }

    // Duplicate Mutable Account Vulnerability: offer_a と offer_b が同じアカウントであるかチェックしない
    pub fn fulfill_offers(ctx: Context<FulfillOffers>) -> Result<()> {
        let offer_a = &mut ctx.accounts.offer_a;
        let offer_b = &mut ctx.accounts.offer_b;

        if !ctx.accounts.market.is_active {
            return Err(ErrorCode::MarketInactive.into());
        }

        let mut total_price_change = 0;
        let mut loop_count = 0;

        while loop_count < 5 {
            if !offer_a.is_fulfilled && !offer_b.is_fulfilled {
                offer_a.price = offer_a.price.checked_add(100).unwrap_or(u64::MAX);
                offer_b.price = offer_b.price.checked_sub(50).unwrap_or(0);
                total_price_change += 50;
                msg!("Adjusting prices for A and B.");
            } else if !offer_a.is_fulfilled {
                offer_a.is_fulfilled = true;
                msg!("Offer A fulfilled.");
                break;
            } else {
                offer_b.is_fulfilled = true;
                msg!("Offer B fulfilled.");
                break;
            }
            loop_count += 1;
        }

        if offer_a.price > 10000 && offer_b.price < 500 {
            offer_a.is_fulfilled = true;
            offer_b.is_fulfilled = true;
            msg!("Offers A and B auto-fulfilled due to price conditions.");
        } else {
            msg!("Conditions not met for auto-fulfillment.");
        }
        
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitMarket<'info> {
    #[account(init, payer = owner, space = 8 + 32 + 32 + 1 + 4)]
    pub market: Account<'info, TradingMarket>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct InitOffer<'info> {
    #[account(mut, has_one = market)]
    pub market: Account<'info, TradingMarket>,
    #[account(init, payer = lister, space = 8 + 32 + 8 + 32 + 8 + 1)]
    pub offer: Account<'info, ItemOffer>,
    #[account(mut)]
    pub lister: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct FulfillOffers<'info> {
    #[account(mut)]
    pub market: Account<'info, TradingMarket>,
    #[account(mut, has_one = market)]
    pub offer_a: Account<'info, ItemOffer>,
    #[account(mut, has_one = market)]
    pub offer_b: Account<'info, ItemOffer>,
}

#[account]
pub struct TradingMarket {
    pub owner: Pubkey,
    pub name: String,
    pub is_active: bool,
    pub trade_count: u32,
}

#[account]
pub struct ItemOffer {
    pub market: Pubkey,
    pub offer_id: u64,
    pub lister: Pubkey,
    pub price: u64,
    pub is_fulfilled: bool,
}

#[error_code]
pub enum MarketError {
    #[msg("The trading market is currently inactive.")]
    MarketInactive,
}

