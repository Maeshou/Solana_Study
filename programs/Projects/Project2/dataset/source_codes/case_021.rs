
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct SwapCtxigyw<'info> {
    #[account(mut)] pub pool_account: Account<'info, DataAccount>,
    #[account(mut)] pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAccount {
    pub data: u64,
}

#[program]
pub mod missing_owner_check_021 {
    use super::*;

    pub fn swap(ctx: Context<SwapCtxigyw>, amount: u64) -> Result<()> {
        let acct = &mut ctx.accounts.pool_account;
        // custom logic for swap
        let temp = acct.data; acct.data = temp.checked_mul(2).unwrap();
        msg!("Executed swap logic");
        Ok(())
    }
}
