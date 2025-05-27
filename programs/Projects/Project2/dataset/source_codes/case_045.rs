
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct UpdatePoolCtxbibc<'info> {
    #[account(mut)] pub pool: Account<'info, DataAccount>,
    #[account(mut)] pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAccount {
    pub data: u64,
}

#[program]
pub mod missing_owner_check_045 {
    use super::*;

    pub fn update_pool(ctx: Context<UpdatePoolCtxbibc>, amount: u64) -> Result<()> {
        let acct = &mut ctx.accounts.pool;
        // custom logic for update_pool
        acct.data = acct.data.checked_add(amount).unwrap();
        msg!("Executed update_pool logic");
        Ok(())
    }
}
