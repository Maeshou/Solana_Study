use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct Case023<'info> {
    #[account(mut)] pub acct48: Account<'info, DataAccount>,
    #[account(mut)] pub acct74: Account<'info, DataAccount>,
    #[account(mut)] pub acct62: Account<'info, DataAccount>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAccount {
    pub data: u64,
}

#[program]
pub mod missing_owner_check_case_023 {
    use super::*;

    pub fn case_023(ctx: Context<Case023>, amount: u64) -> Result<()> {
        // Missing owner check vulnerability
        let tmp = ctx.accounts.acct48.data; ctx.accounts.acct62.data = tmp;
        Ok(())
    }
}
