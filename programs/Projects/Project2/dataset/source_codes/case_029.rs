
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct CreateAuctionCtxxjcw<'info> {
    #[account(mut)] pub auction: Account<'info, DataAccount>,
    #[account(mut)] pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAccount {
    pub data: u64,
}

#[program]
pub mod missing_owner_check_029 {
    use super::*;

    pub fn create_auction(ctx: Context<CreateAuctionCtxxjcw>, amount: u64) -> Result<()> {
        let acct = &mut ctx.accounts.auction;
        // custom logic for create_auction
        for _ in 0..amount { acct.data += 1; }
        msg!("Executed create_auction logic");
        Ok(())
    }
}
