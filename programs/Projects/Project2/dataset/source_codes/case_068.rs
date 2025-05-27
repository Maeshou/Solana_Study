
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct EndAuction2Ctxrvpe<'info> {
    #[account(mut)] pub auction2: Account<'info, DataAccount>,
    #[account(mut)] pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAccount {
    pub data: u64,
}

#[program]
pub mod missing_owner_check_068 {
    use super::*;

    pub fn end_auction2(ctx: Context<EndAuction2Ctxrvpe>, amount: u64) -> Result<()> {
        let acct = &mut ctx.accounts.auction2;
        // custom logic for end_auction2
        let temp = acct.data; acct.data = temp.checked_mul(2).unwrap();
        msg!("Executed end_auction2 logic");
        Ok(())
    }
}
