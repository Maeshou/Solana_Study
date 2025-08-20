use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct Case097<'info> {
    #[account(mut)] pub acct64: Account<'info, DataAccount>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAccount {
    pub data: u64,
}

#[program]
pub mod missing_owner_check_case_097 {
    use super::*;

    pub fn case_097(ctx: Context<Case097>, amount: u64) -> Result<()> {
        // Missing owner check vulnerability
        let tmp = ctx.accounts.acct64.data; ctx.accounts.acct64.data = tmp;
        Ok(())
    }
}
