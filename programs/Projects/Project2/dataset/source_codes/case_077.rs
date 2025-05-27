
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct RemoveLiquidity3Ctxzkyx<'info> {
    #[account(mut)] pub pool3: Account<'info, DataAccount>,
    #[account(mut)] pub provider: Account<'info, DataAccount>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAccount {
    pub data: u64,
}

#[program]
pub mod missing_owner_check_077 {
    use super::*;

    pub fn remove_liquidity3(ctx: Context<RemoveLiquidity3Ctxzkyx>, amount: u64) -> Result<()> {
        let acct = &mut ctx.accounts.pool3;
        // custom logic for remove_liquidity3
        for _ in 0..amount { acct.data += 1; }
        msg!("Executed remove_liquidity3 logic");
        Ok(())
    }
}
