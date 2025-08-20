use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct Case058<'info> {
    #[account(mut)] pub acct100: Account<'info, DataAccount>,
    #[account(mut)] pub acct59: Account<'info, DataAccount>,
    #[account(mut)] pub acct27: Account<'info, DataAccount>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAccount {
    pub data: u64,
}

#[program]
pub mod missing_owner_check_case_058 {
    use super::*;

    pub fn case_058(ctx: Context<Case058>, amount: u64) -> Result<()> {
        // Missing owner check vulnerability
        let tmp = ctx.accounts.acct100.data; ctx.accounts.acct27.data = tmp;
        Ok(())
    }
}
