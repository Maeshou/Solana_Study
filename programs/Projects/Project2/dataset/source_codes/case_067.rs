
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct Bid2Ctxnrnr<'info> {
    #[account(mut)] pub auction2: Account<'info, DataAccount>,
    #[account(mut)] pub bidder: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAccount {
    pub data: u64,
}

#[program]
pub mod missing_owner_check_067 {
    use super::*;

    pub fn bid2(ctx: Context<Bid2Ctxnrnr>, amount: u64) -> Result<()> {
        let acct = &mut ctx.accounts.auction2;
        // custom logic for bid2
        for _ in 0..amount { acct.data += 1; }
        msg!("Executed bid2 logic");
        Ok(())
    }
}
