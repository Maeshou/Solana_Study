use anchor_lang::prelude::*;
use auction_program::cpi::accounts::Bid;


declare_id!("REPLACE_WITH_PROGRAM_ID");

#[program]
pub mod case_026 {
    use super::*;
    pub fn place_bid(ctx: Context<BidVuln026>, bid_amount: u64) -> Result<()> {
        let cpi_program = ctx.accounts.auction_prog.to_account_info();
        let cpi_accounts = Bid {
            auction: ctx.accounts.auction_acc.to_account_info(),
            bidder: ctx.accounts.bidder.to_account_info(),
            system_program: ctx.accounts.system.to_account_info(),
        };
        // Arbitrary CPI
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        auction_program::cpi::bid(cpi_ctx, bid_amount)?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct BidVuln026<'info> {
    #[account(mut)] pub auction_acc: AccountInfo<'info>,
    pub bidder: Signer<'info>,
    pub system: Program<'info, System>,
    /// CHECK: unchecked auction program
    pub auction_prog: UncheckedAccount<'info>,
}