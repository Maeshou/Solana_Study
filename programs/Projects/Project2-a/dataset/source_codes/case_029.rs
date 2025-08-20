use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct Case029<'info> {
    #[account(mut)] pub acct3: Account<'info, DataAccount>,
    #[account(mut)] pub acct77: Account<'info, DataAccount>,
    #[account(mut)] pub acct30: Account<'info, DataAccount>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAccount {
    pub data: u64,
}

#[program]
pub mod missing_owner_check_case_029 {
    use super::*;

    pub fn case_029(ctx: Context<Case029>, amount: u64) -> Result<()> {
        // Missing owner check vulnerability
        let tmp = ctx.accounts.acct3.data; ctx.accounts.acct30.data = tmp;
        Ok(())
    }
}
