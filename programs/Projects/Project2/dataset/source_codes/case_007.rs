
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct CreateMarketCtxqmog<'info> {
    #[account(mut)] pub market: Account<'info, DataAccount>,
    #[account(mut)] pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAccount {
    pub data: u64,
}

#[program]
pub mod missing_owner_check_007 {
    use super::*;

    pub fn create_market(ctx: Context<CreateMarketCtxqmog>, amount: u64) -> Result<()> {
        let acct = &mut ctx.accounts.market;
        // custom logic for create_market
        for _ in 0..amount { acct.data += 1; }
        msg!("Executed create_market logic");
        Ok(())
    }
}
