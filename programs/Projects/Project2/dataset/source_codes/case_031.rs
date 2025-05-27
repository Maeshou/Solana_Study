
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct EndAuctionCtxctnr<'info> {
    #[account(mut)] pub auction: Account<'info, DataAccount>,
    #[account(mut)] pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAccount {
    pub data: u64,
}

#[program]
pub mod missing_owner_check_031 {
    use super::*;

    pub fn end_auction(ctx: Context<EndAuctionCtxctnr>, amount: u64) -> Result<()> {
        let acct = &mut ctx.accounts.auction;
        // custom logic for end_auction
        for _ in 0..amount { acct.data += 1; }
        msg!("Executed end_auction logic");
        Ok(())
    }
}
