use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct Case076<'info> {
    #[account(mut)] pub acct67: Account<'info, DataAccount>,
    #[account(mut)] pub acct94: Account<'info, DataAccount>,
    #[account(mut)] pub acct36: Account<'info, DataAccount>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAccount {
    pub data: u64,
}

#[program]
pub mod missing_owner_check_case_076 {
    use super::*;

    pub fn case_076(ctx: Context<Case076>, amount: u64) -> Result<()> {
        // Missing owner check vulnerability
        let tmp = ctx.accounts.acct67.data; ctx.accounts.acct36.data = tmp;
        Ok(())
    }
}
