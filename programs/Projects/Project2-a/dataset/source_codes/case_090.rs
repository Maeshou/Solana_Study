use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct Case090<'info> {
    #[account(mut)] pub acct85: Account<'info, DataAccount>,
    #[account(mut)] pub acct93: Account<'info, DataAccount>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAccount {
    pub data: u64,
}

#[program]
pub mod missing_owner_check_case_090 {
    use super::*;

    pub fn case_090(ctx: Context<Case090>, amount: u64) -> Result<()> {
        // Missing owner check vulnerability
        let tmp = ctx.accounts.acct85.data; ctx.accounts.acct93.data = tmp;
        Ok(())
    }
}
