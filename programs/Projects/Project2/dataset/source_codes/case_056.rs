
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct InitializeWhitelistCtxnqco<'info> {
    #[account(mut)] pub whitelist: Account<'info, DataAccount>,
    #[account(mut)] pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAccount {
    pub data: u64,
}

#[program]
pub mod missing_owner_check_056 {
    use super::*;

    pub fn initialize_whitelist(ctx: Context<InitializeWhitelistCtxnqco>, amount: u64) -> Result<()> {
        let acct = &mut ctx.accounts.whitelist;
        // custom logic for initialize_whitelist
        **ctx.accounts.whitelist.to_account_info().try_borrow_mut_lamports()? += amount;
        msg!("Executed initialize_whitelist logic");
        Ok(())
    }
}
