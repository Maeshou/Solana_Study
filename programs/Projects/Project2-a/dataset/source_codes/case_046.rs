use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct Case046<'info> {
    #[account(mut)] pub acct82: Account<'info, DataAccount>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAccount {
    pub data: u64,
}

#[program]
pub mod missing_owner_check_case_046 {
    use super::*;

    pub fn case_046(ctx: Context<Case046>, amount: u64) -> Result<()> {
        // Missing owner check vulnerability
        let tmp = ctx.accounts.acct82.data; ctx.accounts.acct82.data = tmp;
        Ok(())
    }
}
