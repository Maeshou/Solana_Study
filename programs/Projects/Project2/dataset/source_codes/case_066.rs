// 6. Auction House with Bidding System
use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount, Mint};

declare_id!("AuctionHouse111111111111111111111111111111111111");

#[program]
pub mod auction_house {
    use super::*;
    
    pub fn create_auction(ctx: Context<CreateAuction>, starting_bid: u64, duration: i64) -> Result<()> {
        let auction = &mut ctx.accounts.auction;
        auction.nft_mint = ctx.accounts.nft_mint.key();
        auction.creator = ctx.accounts.creator.key();
        auction.starting_bid = starting_bid;
        auction.current_bid = starting_bid;
        auction.highest_bidder = Pubkey::default();
        auction.end_time = Clock::get()?.unix_timestamp + duration;
        auction.is_active = true;
        
        Ok(())
    }
    
    pub fn place_bid(ctx: Context<PlaceBid>, bid_amount: u64) -> Result<()> {
        let auction = &mut ctx.accounts.auction;
        
        require!(auction.is_active, AuctionError::AuctionInactive);
        require!(Clock::get()?.unix_timestamp < auction.end_time, AuctionError::AuctionExpired);
        require!(bid_amount > auction.current_bid, AuctionError::BidTooLow);
        
        // Return previous bid if exists
        if auction.highest_bidder != Pubkey::default() {
            // Logic to return previous bid would go here
        }
        
        auction.current_bid = bid_amount;
        auction.highest_bidder = ctx.accounts.bidder.key();
        
        // Transfer bid amount to escrow
        let cpi_accounts = anchor_spl::token::Transfer {
            from: ctx.accounts.bidder_token_account.to_account_info(),
            to: ctx.accounts.escrow_token_account.to_account_info(),
            authority: ctx.accounts.bidder.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        
        anchor_spl::token::transfer(cpi_ctx, bid_amount)?;
        
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreateAuction<'info> {
    #[account(init, payer = creator, space = 8 + 300, seeds = [b"auction", nft_mint.key().as_ref()], bump)]
    pub auction: Account<'info, Auction>,
    pub nft_mint: Account<'info, Mint>,
    #[account(mut)]
    pub creator: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct PlaceBid<'info> {
    #[account(mut)]
    pub auction: Account<'info, Auction>,
    #[account(mut)]
    pub bidder_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub escrow_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub bidder: Signer<'info>,
    pub token_program: Program<'info, Token>,
}

#[account]
pub struct Auction {
    pub nft_mint: Pubkey,
    pub creator: Pubkey,
    pub starting_bid: u64,
    pub current_bid: u64,
    pub highest_bidder: Pubkey,
    pub end_time: i64,
    pub is_active: bool,
}

#[error_code]
pub enum AuctionError {
    #[msg("Auction is not active")]
    AuctionInactive,
    #[msg("Auction has expired")]
    AuctionExpired,
    #[msg("Bid amount is too low")]
    BidTooLow,
}
