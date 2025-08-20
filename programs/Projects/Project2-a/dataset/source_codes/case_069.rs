use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct Case069<'info> {
    #[account(mut)] pub acct86: Account<'info, DataAccount>,
    #[account(mut)] pub acct89: Account<'info, DataAccount>,
    #[account(mut)] pub acct36: Account<'info, DataAccount>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAccount {
    pub data: u64,
}

#[program]
pub mod missing_owner_check_case_069 {
    use super::*;

    pub fn case_069(ctx: Context<Case069>, amount: u64) -> Result<()> {
        // Missing owner check vulnerability
        let tmp = ctx.accounts.acct86.data; ctx.accounts.acct36.data = tmp;
        Ok(())
    }
}
