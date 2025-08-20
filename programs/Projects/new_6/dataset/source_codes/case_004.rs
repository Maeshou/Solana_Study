
// ========================================
// 4. 脆弱なオークション - Vulnerable Auction House
// ========================================

use anchor_lang::prelude::*;

declare_id!("V4uLnErAbLeCoD3F0r3xAmP1e5tUdY7BaTt1eAr3nA3x");

#[program]
pub mod vulnerable_auction {
    use super::*;

    pub fn init_auction_house(ctx: Context<InitAuctionHouse>) -> Result<()> {
        let house = &mut ctx.accounts.auction_house;
        house.operator = ctx.accounts.operator.key();
        house.total_auctions = 0;
        house.commission_rate = 10; // 10%
        Ok(())
    }

    pub fn create_auction(ctx: Context<CreateAuction>, starting_price: u64) -> Result<()> {
        let auction = &mut ctx.accounts.auction;
        auction.house = ctx.accounts.auction_house.key();
        auction.seller = ctx.accounts.seller.key();
        auction.starting_price = starting_price;
        auction.current_bid = starting_price;
        auction.highest_bidder = Pubkey::default();
        auction.active = true;

        let house = &mut ctx.accounts.auction_house;
        house.total_auctions = house.total_auctions.checked_add(1).unwrap_or(u64::MAX);
        Ok(())
    }

    // 脆弱性: invoke_signedの直接使用とAccountInfo
    pub fn vulnerable_bid(ctx: Context<VulnerableBid>) -> Result<()> {
        let house = &mut ctx.accounts.auction_house;
        
        // 脆弱性: AccountInfoで型安全性なし
        let auction_info = &ctx.accounts.auction_account;
        let bidder_info = &ctx.accounts.bidder_account;

        // 脆弱性: 手動でデータ解析、discriminator検証なし
        let auction_data = auction_info.try_borrow_mut_data()?;
        if auction_data.len() >= 80 {
            let mut current_bid_bytes = [0u8; 8];
            current_bid_bytes.copy_from_slice(&auction_data[72..80]);
            let mut current_bid = u64::from_le_bytes(current_bid_bytes);

            // 入札処理ループ
            for bid_round in 0..6 {
                if current_bid < 10000 {
                    let bid_increment = (bid_round + 1) as u64 * 100;
                    current_bid = current_bid.checked_add(bid_increment).unwrap_or(u64::MAX);
                    
                    // 脆弱性: 直接データ操作
                    let new_bid_bytes = current_bid.to_le_bytes();
                    auction_data[72..80].copy_from_slice(&new_bid_bytes);
                    
                    house.commission_rate = (house.commission_rate + bid_round as u32).min(25);
                    msg!("Bid round {}: new bid = {}", bid_round, current_bid);
                } else {
                    let commission = current_bid / 10;
                    house.total_auctions = house.total_auctions.checked_add(commission).unwrap_or(u64::MAX);
                    
                    // ビット操作による価格調整
                    current_bid = (current_bid >> 1) | 0x8000_0000_0000_0000;
                    let adjusted_bytes = current_bid.to_le_bytes();
                    auction_data[72..80].copy_from_slice(&adjusted_bytes);
                    msg!("Commission collected: {}", commission);
                }
            }
        }

        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitAuctionHouse<'info> {
    #[account(init, payer = operator, space = 8 + 32 + 8 + 4)]
    pub auction_house: Account<'info, AuctionHouse>,
    #[account(mut)]
    pub operator: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CreateAuction<'info> {
    #[account(mut)]
    pub auction_house: Account<'info, AuctionHouse>,
    #[account(init, payer = seller, space = 8 + 32 + 32 + 8 + 8 + 32 + 1)]
    pub auction: Account<'info, Auction>,
    #[account(mut)]
    pub seller: Signer<'info>,
    pub system_program: Program<'info, System>,
}

// 脆弱性: AccountInfoの直接使用
#[derive(Accounts)]
pub struct VulnerableBid<'info> {
    #[account(mut)]
    pub auction_house: Account<'info, AuctionHouse>,
    /// CHECK: 脆弱性 - 型検証なしのAccountInfo
    pub auction_account: AccountInfo<'info>,
    /// CHECK: 脆弱性 - 入札者検証なし
    pub bidder_account: AccountInfo<'info>,
    pub bidder: Signer<'info>,
}

#[account]
pub struct AuctionHouse {
    pub operator: Pubkey,
    pub total_auctions: u64,
    pub commission_rate: u32,
}

#[account]
pub struct Auction {
    pub house: Pubkey,
    pub seller: Pubkey,
    pub starting_price: u64,
    pub current_bid: u64,
    pub highest_bidder: Pubkey,
    pub active: bool,
}