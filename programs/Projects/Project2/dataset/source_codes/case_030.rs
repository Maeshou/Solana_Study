
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct BidCtxfdsg<'info> {
    #[account(mut)] pub auction: Account<'info, DataAccount>,
    #[account(mut)] pub bidder: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAccount {
    pub data: u64,
}

#[program]
pub mod missing_owner_check_030 {
    use super::*;

    pub fn bid(ctx: Context<BidCtxfdsg>, amount: u64) -> Result<()> {
        let acct = &mut ctx.accounts.auction;
        // custom logic for bid
        let temp = acct.data; acct.data = temp.checked_mul(2).unwrap();
        msg!("Executed bid logic");
        Ok(())
    }
}
