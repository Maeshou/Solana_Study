use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct Case033<'info> {
    #[account(mut)] pub acct71: Account<'info, DataAccount>,
    #[account(mut)] pub acct35: Account<'info, DataAccount>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAccount {
    pub data: u64,
}

#[program]
pub mod missing_owner_check_case_033 {
    use super::*;

    pub fn case_033(ctx: Context<Case033>, amount: u64) -> Result<()> {
        // Missing owner check vulnerability
        let tmp = ctx.accounts.acct71.data; ctx.accounts.acct35.data = tmp;
        Ok(())
    }
}
