
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct AddLiquidityCtxmafh<'info> {
    #[account(mut)] pub pool_account: Account<'info, DataAccount>,
    #[account(mut)] pub provider: Account<'info, DataAccount>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAccount {
    pub data: u64,
}

#[program]
pub mod missing_owner_check_019 {
    use super::*;

    pub fn add_liquidity(ctx: Context<AddLiquidityCtxmafh>, amount: u64) -> Result<()> {
        let acct = &mut ctx.accounts.pool_account;
        // custom logic for add_liquidity
        let temp = acct.data; acct.data = temp.checked_mul(2).unwrap();
        msg!("Executed add_liquidity logic");
        Ok(())
    }
}
