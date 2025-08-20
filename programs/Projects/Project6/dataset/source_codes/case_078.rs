use anchor_lang::prelude::*;

declare_id!("AUCTI888888888888888888888888888888888888888");

#[program]
pub mod auction_house_program {
    use super::*;
    /// 現在価格より高い額で入札し、最高入札者情報を更新します。
    pub fn place_bid_on_auction(ctx: Context<PlaceBid>, bid_amount: u64) -> Result<()> {
        let auction = &mut ctx.accounts.auction_state;
        
        // 新しい入札者から資金を受け取る
        let cpi_accounts = anchor_lang::system_program::Transfer {
            from: ctx.accounts.bidder.to_account_info(),
            to: ctx.accounts.auction_vault.to_account_info(),
        };
        let cpi_program = ctx.accounts.system_program.to_account_info();
        let cpi_context = CpiContext::new(cpi_program, cpi_accounts);
        anchor_lang::system_program::transfer(cpi_context, bid_amount)?;
        
        auction.highest_bid = bid_amount;
        auction.highest_bidder = *ctx.accounts.bidder.key;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct PlaceBid<'info> {
    #[account(mut, constraint = bid_amount > auction_state.highest_bid @ GameErrorCode::BidTooLow, constraint = Clock::get().unwrap().unix_timestamp < auction_state.end_timestamp @ GameErrorCode::AuctionEnded)]
    pub auction_state: Account<'info, Auction>,
    #[account(mut)]
    /// CHECK: This is a simple vault account for demonstration.
    pub auction_vault: AccountInfo<'info>,
    #[account(mut)]
    pub bidder: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Auction {
    pub highest_bidder: Pubkey,
    pub highest_bid: u64,
    pub end_timestamp: i64,
}

#[error_code]
pub enum GameErrorCode {
    #[msg("The auction has already ended.")]
    AuctionEnded,
    #[msg("Bid amount must be higher than the current highest bid.")]
    BidTooLow,
}