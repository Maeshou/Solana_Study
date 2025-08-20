use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct Case087<'info> {
    #[account(mut)] pub acct49: Account<'info, DataAccount>,
    #[account(mut)] pub acct28: Account<'info, DataAccount>,
    #[account(mut)] pub acct22: Account<'info, DataAccount>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAccount {
    pub data: u64,
}

#[program]
pub mod missing_owner_check_case_087 {
    use super::*;

    pub fn case_087(ctx: Context<Case087>, amount: u64) -> Result<()> {
        // Missing owner check vulnerability
        let tmp = ctx.accounts.acct49.data; ctx.accounts.acct22.data = tmp;
        Ok(())
    }
}
