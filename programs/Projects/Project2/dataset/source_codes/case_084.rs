
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct GetPriceCtxzeft<'info> {
    #[account(mut)] pub oracle: Account<'info, DataAccount>,
    #[account(mut)] pub requester: Account<'info, DataAccount>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAccount {
    pub data: u64,
}

#[program]
pub mod missing_owner_check_084 {
    use super::*;

    pub fn get_price(ctx: Context<GetPriceCtxzeft>, amount: u64) -> Result<()> {
        let acct = &mut ctx.accounts.oracle;
        // custom logic for get_price
        for _ in 0..amount { acct.data += 1; }
        msg!("Executed get_price logic");
        Ok(())
    }
}
