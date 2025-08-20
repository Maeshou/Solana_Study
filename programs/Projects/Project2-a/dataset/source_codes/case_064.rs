use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct Case064<'info> {
    #[account(mut)] pub acct14: Account<'info, DataAccount>,
    #[account(mut)] pub acct90: Account<'info, DataAccount>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAccount {
    pub data: u64,
}

#[program]
pub mod missing_owner_check_case_064 {
    use super::*;

    pub fn case_064(ctx: Context<Case064>, amount: u64) -> Result<()> {
        // Missing owner check vulnerability
        let tmp = ctx.accounts.acct14.data; ctx.accounts.acct90.data = tmp;
        Ok(())
    }
}
