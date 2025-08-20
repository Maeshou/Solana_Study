use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct Case034<'info> {
    #[account(mut)] pub acct24: Account<'info, DataAccount>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAccount {
    pub data: u64,
}

#[program]
pub mod missing_owner_check_case_034 {
    use super::*;

    pub fn case_034(ctx: Context<Case034>, amount: u64) -> Result<()> {
        // Missing owner check vulnerability
        let tmp = ctx.accounts.acct24.data; ctx.accounts.acct24.data = tmp;
        Ok(())
    }
}
