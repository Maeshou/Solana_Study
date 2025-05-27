
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct InitializeAccountCtxjiin<'info> {
    #[account(mut)] pub account: Account<'info, DataAccount>,
    #[account(mut)] pub owner: Account<'info, DataAccount>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAccount {
    pub data: u64,
}

#[program]
pub mod missing_owner_check_022 {
    use super::*;

    pub fn initialize_account(ctx: Context<InitializeAccountCtxjiin>, amount: u64) -> Result<()> {
        let acct = &mut ctx.accounts.account;
        // custom logic for initialize_account
        for _ in 0..amount { acct.data += 1; }
        msg!("Executed initialize_account logic");
        Ok(())
    }
}
