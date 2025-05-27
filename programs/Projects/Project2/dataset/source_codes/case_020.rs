
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct RemoveLiquidityCtxgkoq<'info> {
    #[account(mut)] pub pool_account: Account<'info, DataAccount>,
    #[account(mut)] pub provider: Account<'info, DataAccount>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAccount {
    pub data: u64,
}

#[program]
pub mod missing_owner_check_020 {
    use super::*;

    pub fn remove_liquidity(ctx: Context<RemoveLiquidityCtxgkoq>, amount: u64) -> Result<()> {
        let acct = &mut ctx.accounts.pool_account;
        // custom logic for remove_liquidity
        acct.data = acct.data.checked_add(amount).unwrap();
        msg!("Executed remove_liquidity logic");
        Ok(())
    }
}
