
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct CloseAccountCtxbvbs<'info> {
    #[account(mut)] pub account: Account<'info, DataAccount>,
    #[account(mut)] pub owner: Account<'info, DataAccount>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAccount {
    pub data: u64,
}

#[program]
pub mod missing_owner_check_023 {
    use super::*;

    pub fn close_account(ctx: Context<CloseAccountCtxbvbs>, amount: u64) -> Result<()> {
        let acct = &mut ctx.accounts.account;
        // custom logic for close_account
        let temp = acct.data; acct.data = temp.checked_mul(2).unwrap();
        msg!("Executed close_account logic");
        Ok(())
    }
}
