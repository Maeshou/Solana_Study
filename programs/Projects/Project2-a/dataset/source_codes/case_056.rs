use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct Case056<'info> {
    #[account(mut)] pub acct93: Account<'info, DataAccount>,
    #[account(mut)] pub acct63: Account<'info, DataAccount>,
    #[account(mut)] pub acct5: Account<'info, DataAccount>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAccount {
    pub data: u64,
}

#[program]
pub mod missing_owner_check_case_056 {
    use super::*;

    pub fn case_056(ctx: Context<Case056>, amount: u64) -> Result<()> {
        // Missing owner check vulnerability
        let tmp = ctx.accounts.acct93.data; ctx.accounts.acct5.data = tmp;
        Ok(())
    }
}
