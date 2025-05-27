
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct InitPoolCtxbvdv<'info> {
    #[account(mut)] pub pool_account: Account<'info, DataAccount>,
    #[account(mut)] pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAccount {
    pub data: u64,
}

#[program]
pub mod missing_owner_check_018 {
    use super::*;

    pub fn init_pool(ctx: Context<InitPoolCtxbvdv>, amount: u64) -> Result<()> {
        let acct = &mut ctx.accounts.pool_account;
        // custom logic for init_pool
        for _ in 0..amount { acct.data += 1; }
        msg!("Executed init_pool logic");
        Ok(())
    }
}
