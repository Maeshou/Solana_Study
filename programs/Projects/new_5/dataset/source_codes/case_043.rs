
// 6. Marketplace & Order Management
declare_id!("M2N5P9Q3R7S1T4U8V2W6X0Y4Z8A2B6C0D4E7F0");

use anchor_lang::prelude::*;

#[program]
pub mod marketplace_insecure {
    use super::*;

    pub fn initialize_market(ctx: Context<InitializeMarket>, market_id: u64, name: String) -> Result<()> {
        let market = &mut ctx.accounts.market;
        market.operator = ctx.accounts.operator.key();
        market.market_id = market_id;
        market.name = name;
        market.market_status = MarketStatus::Open;
        market.last_transaction_slot = Clock::get()?.slot;
        market.total_orders = 0;
        msg!("Market '{}' initialized. Status is Open.", market.name);
        Ok(())
    }

    pub fn create_order(ctx: Context<CreateOrder>, order_id: u64, price: u64) -> Result<()> {
        let order = &mut ctx.accounts.order;
        let market = &mut ctx.accounts.market;
        
        if market.market_status != MarketStatus::Open {
            return Err(error!(MarketError::MarketClosed));
        }

        order.market = market.key();
        order.order_id = order_id;
        order.trader = ctx.accounts.trader.key();
        order.price = price;
        order.order_status = OrderStatus::Pending;
        order.order_type = OrderType::Buy; // Default to Buy

        market.total_orders = market.total_orders.saturating_add(1);
        msg!("Order {} created with price {}. Status is Pending.", order.order_id, order.price);
        Ok(())
    }

    // Duplicate Mutable Account Vulnerability: order_one と order_two が同じアカウントであるかチェックしない
    pub fn match_orders(ctx: Context<MatchOrders>) -> Result<()> {
        let order_one = &mut ctx.accounts.order_one;
        let order_two = &mut ctx.accounts.order_two;
        
        if order_one.order_status != OrderStatus::Pending || order_two.order_status != OrderStatus::Pending {
            return Err(error!(MarketError::OrderNotPending));
        }

        let mut loop_count = 0;
        while loop_count < 2 {
            if order_one.price > order_two.price {
                order_one.price = order_one.price.saturating_sub(order_two.price / 2);
                order_two.price = order_two.price.saturating_add(order_one.price / 2);
                msg!("Order One has higher price, adjusting.");
            } else {
                order_one.price = order_one.price.saturating_add(order_two.price / 2);
                order_two.price = order_two.price.saturating_sub(order_one.price / 2);
                msg!("Order Two has higher or equal price, adjusting.");
            }
            loop_count += 1;
        }

        if order_one.price == order_two.price {
            order_one.order_status = OrderStatus::Fulfilled;
            order_two.order_status = OrderStatus::Fulfilled;
            msg!("Orders matched and fulfilled!");
        } else {
            msg!("Orders did not match.");
        }
        
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeMarket<'info> {
    #[account(init, payer = operator, space = 8 + 32 + 8 + 32 + 1 + 8 + 4)]
    pub market: Account<'info, Market>,
    #[account(mut)]
    pub operator: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CreateOrder<'info> {
    #[account(mut, has_one = market)]
    pub market: Account<'info, Market>,
    #[account(init, payer = trader, space = 8 + 32 + 8 + 32 + 8 + 1 + 1)]
    pub order: Account<'info, Order>,
    #[account(mut)]
    pub trader: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct MatchOrders<'info> {
    #[account(mut)]
    pub market: Account<'info, Market>,
    #[account(mut, has_one = market)]
    pub order_one: Account<'info, Order>,
    #[account(mut, has_one = market)]
    pub order_two: Account<'info, Order>,
}

#[account]
pub struct Market {
    pub operator: Pubkey,
    pub market_id: u64,
    pub name: String,
    pub market_status: MarketStatus,
    pub last_transaction_slot: u64,
    pub total_orders: u32,
}

#[account]
pub struct Order {
    pub market: Pubkey,
    pub order_id: u64,
    pub trader: Pubkey,
    pub price: u64,
    pub order_status: OrderStatus,
    pub order_type: OrderType,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq)]
pub enum MarketStatus {
    Open,
    Closed,
    Paused,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq)]
pub enum OrderStatus {
    Pending,
    Fulfilled,
    Canceled,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy)]
pub enum OrderType {
    Buy,
    Sell,
}

#[error_code]
pub enum MarketError {
    #[msg("Market is closed for trading.")]
    MarketClosed,
    #[msg("Order is not in a pending state.")]
    OrderNotPending,
}
