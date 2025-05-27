
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct ThawAccountCtxnxzu<'info> {
    #[account(mut)] pub account: Account<'info, DataAccount>,
    #[account(mut)] pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAccount {
    pub data: u64,
}

#[program]
pub mod missing_owner_check_025 {
    use super::*;

    pub fn thaw_account(ctx: Context<ThawAccountCtxnxzu>, amount: u64) -> Result<()> {
        let acct = &mut ctx.accounts.account;
        // custom logic for thaw_account
        **ctx.accounts.account.to_account_info().try_borrow_mut_lamports()? += amount;
        msg!("Executed thaw_account logic");
        Ok(())
    }
}
