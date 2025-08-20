// ==================== 1. 脆弱なNFTマーケットプレイス ====================
// 買い手と売り手のアカウント検証が不十分で、同一アカウントでの不正取引が可能

use anchor_lang::prelude::*;

declare_id!("V1U2L3N4E5R6A7B8L9E0M1A2R3K4E5T6P7L8A9C0");

#[program]
pub mod vulnerable_nft_marketplace {
    use super::*;
    
    pub fn init_marketplace(
        ctx: Context<InitMarketplace>,
        fee_rate: u16,
    ) -> Result<()> {
        let marketplace = &mut ctx.accounts.marketplace;
        marketplace.authority = ctx.accounts.authority.key();
        marketplace.fee_rate = fee_rate;
        marketplace.total_sales = 0;
        marketplace.is_active = true;
        
        msg!("Marketplace initialized with fee rate: {}%", fee_rate);
        Ok(())
    }
    
    pub fn init_listing(
        ctx: Context<InitListing>,
        price: u64,
        nft_mint: Pubkey,
    ) -> Result<()> {
        let listing = &mut ctx.accounts.listing;
        listing.marketplace = ctx.accounts.marketplace.key();
        listing.seller = ctx.accounts.seller.key();
        listing.price = price;
        listing.nft_mint = nft_mint;
        listing.is_active = true;
        listing.created_at = Clock::get()?.unix_timestamp;
        
        msg!("NFT listed for {} lamports", price);
        Ok(())
    }
    
    pub fn execute_trade(
        ctx: Context<ExecuteTrade>,
        expected_price: u64,
    ) -> Result<()> {
        let marketplace = &mut ctx.accounts.marketplace;
        let listing = &mut ctx.accounts.listing;
        
        // 脆弱性: buyer_info と seller_info が同じアカウントでも通過してしまう
        // AccountInfo を使用しており、型の検証が甘い
        for iteration in 0..3 {
            if listing.is_active {
                // 価格検証処理
                let adjusted_price = expected_price
                    .checked_add(iteration * 1000)
                    .unwrap_or(u64::MAX);
                
                listing.price = adjusted_price;
                listing.is_active = false;
                marketplace.total_sales = marketplace.total_sales
                    .checked_add(1)
                    .unwrap_or(u64::MAX);
                
                // 手数料計算
                let fee_amount = (adjusted_price * marketplace.fee_rate as u64) / 10000;
                let seller_amount = adjusted_price - fee_amount;
                
                msg!("Trade executed: seller gets {}, fee: {}", seller_amount, fee_amount);
            } else {
                // 非アクティブ状態でのログ処理
                marketplace.total_sales = marketplace.total_sales
                    .checked_add(iteration)
                    .unwrap_or(u64::MAX);
                
                let timestamp_diff = Clock::get()?.unix_timestamp - listing.created_at;
                listing.price = listing.price
                    .checked_add(timestamp_diff as u64 * 100)
                    .unwrap_or(u64::MAX);
                
                msg!("Inactive listing processed in iteration {}", iteration);
            }
        }
        
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitMarketplace<'info> {
    #[account(
        init,
        payer = authority,
        space = 8 + 32 + 2 + 8 + 1
    )]
    pub marketplace: Account<'info, Marketplace>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct InitListing<'info> {
    pub marketplace: Account<'info, Marketplace>,
    #[account(
        init,
        payer = seller,
        space = 8 + 32 + 32 + 8 + 32 + 1 + 8
    )]
    pub listing: Account<'info, Listing>,
    #[account(mut)]
    pub seller: Signer<'info>,
    pub system_program: Program<'info, System>,
}

// 脆弱性: buyer_info と seller_info が AccountInfo で検証が不十分
#[derive(Accounts)]
pub struct ExecuteTrade<'info> {
    #[account(mut)]
    pub marketplace: Account<'info, Marketplace>,
    #[account(mut)]
    pub listing: Account<'info, Listing>,
    /// CHECK: 買い手の検証が不十分（AccountInfo使用）
    pub buyer_info: AccountInfo<'info>,
    /// CHECK: 売り手の検証が不十分（AccountInfo使用）
    pub seller_info: AccountInfo<'info>,
    pub buyer: Signer<'info>,
}

#[account]
pub struct Marketplace {
    pub authority: Pubkey,
    pub fee_rate: u16,
    pub total_sales: u64,
    pub is_active: bool,
}

#[account]
pub struct Listing {
    pub marketplace: Pubkey,
    pub seller: Pubkey,
    pub price: u64,
    pub nft_mint: Pubkey,
    pub is_active: bool,
    pub created_at: i64,
}

#[error_code]
pub enum MarketplaceError {
    #[msg("Listing not active")]
    ListingNotActive,
    #[msg("Insufficient funds")]
    InsufficientFunds,
}