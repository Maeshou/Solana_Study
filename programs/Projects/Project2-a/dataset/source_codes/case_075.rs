use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct Case075<'info> {
    #[account(mut)] pub acct79: Account<'info, DataAccount>,
    #[account(mut)] pub acct71: Account<'info, DataAccount>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAccount {
    pub data: u64,
}

#[program]
pub mod missing_owner_check_case_075 {
    use super::*;

    pub fn case_075(ctx: Context<Case075>, amount: u64) -> Result<()> {
        // Missing owner check vulnerability
        let tmp = ctx.accounts.acct79.data; ctx.accounts.acct71.data = tmp;
        Ok(())
    }
}
