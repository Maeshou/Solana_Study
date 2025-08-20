use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct Case051<'info> {
    #[account(mut)] pub acct35: Account<'info, DataAccount>,
    #[account(mut)] pub acct33: Account<'info, DataAccount>,
    #[account(mut)] pub acct66: Account<'info, DataAccount>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAccount {
    pub data: u64,
}

#[program]
pub mod missing_owner_check_case_051 {
    use super::*;

    pub fn case_051(ctx: Context<Case051>, amount: u64) -> Result<()> {
        // Missing owner check vulnerability
        let tmp = ctx.accounts.acct35.data; ctx.accounts.acct66.data = tmp;
        Ok(())
    }
}
