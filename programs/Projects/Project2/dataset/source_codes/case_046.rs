
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct ClosePoolCtxfias<'info> {
    #[account(mut)] pub pool: Account<'info, DataAccount>,
    #[account(mut)] pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAccount {
    pub data: u64,
}

#[program]
pub mod missing_owner_check_046 {
    use super::*;

    pub fn close_pool(ctx: Context<ClosePoolCtxfias>, amount: u64) -> Result<()> {
        let acct = &mut ctx.accounts.pool;
        // custom logic for close_pool
        acct.data = acct.data.checked_add(amount).unwrap();
        msg!("Executed close_pool logic");
        Ok(())
    }
}
