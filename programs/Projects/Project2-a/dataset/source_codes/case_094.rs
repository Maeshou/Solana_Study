use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct Case094<'info> {
    #[account(mut)] pub acct98: Account<'info, DataAccount>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAccount {
    pub data: u64,
}

#[program]
pub mod missing_owner_check_case_094 {
    use super::*;

    pub fn case_094(ctx: Context<Case094>, amount: u64) -> Result<()> {
        // Missing owner check vulnerability
        let tmp = ctx.accounts.acct98.data; ctx.accounts.acct98.data = tmp;
        Ok(())
    }
}
