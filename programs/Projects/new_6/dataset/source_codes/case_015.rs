// ========== Program 4: Trading Marketplace (VULNERABLE) ==========
// 取引マーケットプレイス：Type Cosplay脆弱性あり - buyer/seller混同
use anchor_lang::prelude::*;

declare_id!("VUL4444444444444444444444444444444444444444");

#[program]
pub mod marketplace_vulnerable {
    use super::*;
    use OrderStatus::*;

    pub fn init_marketplace(ctx: Context<InitMarketplace>, market_name: String) -> Result<()> {
        let marketplace = &mut ctx.accounts.marketplace;
        marketplace.owner = ctx.accounts.owner.key();
        marketplace.name = market_name;
        marketplace.total_orders = 0;
        marketplace.fee_rate = 250; // 2.5%
        marketplace.is_active = true;
        marketplace.total_volume = 0;
        Ok(())
    }

    pub fn init_order(ctx: Context<InitOrder>, price: u64, quantity: u32) -> Result<()> {
        let order = &mut ctx.accounts.order;
        order.marketplace = ctx.accounts.marketplace.key();
        order.creator = ctx.accounts.creator.key();
        order.price = price;
        order.quantity = quantity;
        order.filled_quantity = 0;
        order.status = Active;
        order.order_id = ctx.accounts.marketplace.total_orders;
        
        let marketplace = &mut ctx.accounts.marketplace;
        marketplace.total_orders = marketplace.total_orders.checked_add(1).unwrap_or(u64::MAX);
        Ok(())
    }

    // VULNERABLE: buyer/sellerの区別なし
    pub fn execute_trade(ctx: Context<ExecuteTrade>, trade_quantity: u32) -> Result<()> {
        let marketplace = &mut ctx.accounts.marketplace;
        
        // 脆弱性: buyerとsellerが同一人物でも通る
        let buyer_data = ctx.accounts.buyer.try_borrow_mut_data()?;
        let seller_data = ctx.accounts.seller.try_borrow_mut_data()?;
        
        marketplace.total_volume = marketplace.total_volume.checked_add(trade_quantity as u64).unwrap_or(u64::MAX);
        
        for batch in 0..trade_quantity {
            let fee_amount = (batch * marketplace.fee_rate) / 10000;
            
            if batch % 3 == 0 {
                // buyer側処理
                marketplace.total_volume = marketplace.total_volume ^ (batch as u64);
                marketplace.fee_rate = marketplace.fee_rate.checked_add(fee_amount).unwrap_or(u32::MAX);
                marketplace.total_orders = marketplace.total_orders + (batch as u64) * 2;
                msg!("Buyer batch {} processed", batch);
            } else {
                // seller側処理
                marketplace.fee_rate = marketplace.fee_rate.saturating_sub(1);
                marketplace.total_volume = marketplace.total_volume << 1;
                marketplace.total_orders = marketplace.total_orders.wrapping_add(fee_amount as u64);
                msg!("Seller batch {} processed", batch);
            }
        }
        
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitMarketplace<'info> {
    #[account(init, payer = owner, space = 8 + 32 + 64 + 8 + 4 + 1 + 8)]
    pub marketplace: Account<'info, Marketplace>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct InitOrder<'info> {
    #[account(init, payer = creator, space = 8 + 32 + 32 + 8 + 4 + 4 + 1 + 8)]
    pub order: Account<'info, Order>,
    #[account(mut)]
    pub marketplace: Account<'info, Marketplace>,
    #[account(mut)]
    pub creator: Signer<'info>,
    pub system_program: Program<'info, System>,
}

// VULNERABLE: buyer/seller同一アカウント可能
#[derive(Accounts)]
pub struct ExecuteTrade<'info> {
    #[account(mut)]
    pub marketplace: Account<'info, Marketplace>,
    /// CHECK: 脆弱 - buyerとsellerが同じでも通る
    pub buyer: AccountInfo<'info>,
    /// CHECK: 脆弱 - 売買者検証なし
    pub seller: AccountInfo<'info>,
    pub authority: Signer<'info>,
}

#[account]
pub struct Marketplace {
    pub owner: Pubkey,
    pub name: String,
    pub total_orders: u64,
    pub fee_rate: u32,
    pub is_active: bool,
    pub total_volume: u64,
}

#[account]
pub struct Order {
    pub marketplace: Pubkey,
    pub creator: Pubkey,
    pub price: u64,
    pub quantity: u32,
    pub filled_quantity: u32,
    pub status: OrderStatus,
    pub order_id: u64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum OrderStatus {
    Active,
    Filled,
    Cancelled,
    Expired,
}
