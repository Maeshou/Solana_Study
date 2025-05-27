
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct InitializeAuction2Ctxgppf<'info> {
    #[account(mut)] pub auction2: Account<'info, DataAccount>,
    #[account(mut)] pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAccount {
    pub data: u64,
}

#[program]
pub mod missing_owner_check_066 {
    use super::*;

    pub fn initialize_auction2(ctx: Context<InitializeAuction2Ctxgppf>, amount: u64) -> Result<()> {
        let acct = &mut ctx.accounts.auction2;
        // custom logic for initialize_auction2
        let temp = acct.data; acct.data = temp.checked_mul(2).unwrap();
        msg!("Executed initialize_auction2 logic");
        Ok(())
    }
}
