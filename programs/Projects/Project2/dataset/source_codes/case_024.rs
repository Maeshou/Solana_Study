
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct FreezeAccountCtxqnrf<'info> {
    #[account(mut)] pub account: Account<'info, DataAccount>,
    #[account(mut)] pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAccount {
    pub data: u64,
}

#[program]
pub mod missing_owner_check_024 {
    use super::*;

    pub fn freeze_account(ctx: Context<FreezeAccountCtxqnrf>, amount: u64) -> Result<()> {
        let acct = &mut ctx.accounts.account;
        // custom logic for freeze_account
        let temp = acct.data; acct.data = temp.checked_mul(2).unwrap();
        msg!("Executed freeze_account logic");
        Ok(())
    }
}
