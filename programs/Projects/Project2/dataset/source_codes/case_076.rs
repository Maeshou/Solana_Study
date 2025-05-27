
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct AddLiquidity3Ctxnzmx<'info> {
    #[account(mut)] pub pool3: Account<'info, DataAccount>,
    #[account(mut)] pub provider: Account<'info, DataAccount>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAccount {
    pub data: u64,
}

#[program]
pub mod missing_owner_check_076 {
    use super::*;

    pub fn add_liquidity3(ctx: Context<AddLiquidity3Ctxnzmx>, amount: u64) -> Result<()> {
        let acct = &mut ctx.accounts.pool3;
        // custom logic for add_liquidity3
        assert!(ctx.accounts.pool3.data > 0); acct.data -= amount;
        msg!("Executed add_liquidity3 logic");
        Ok(())
    }
}
