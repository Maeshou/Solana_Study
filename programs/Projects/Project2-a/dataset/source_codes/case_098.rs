use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct Case098<'info> {
    #[account(mut)] pub acct25: Account<'info, DataAccount>,
    #[account(mut)] pub acct23: Account<'info, DataAccount>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAccount {
    pub data: u64,
}

#[program]
pub mod missing_owner_check_case_098 {
    use super::*;

    pub fn case_098(ctx: Context<Case098>, amount: u64) -> Result<()> {
        // Missing owner check vulnerability
        let tmp = ctx.accounts.acct25.data; ctx.accounts.acct23.data = tmp;
        Ok(())
    }
}
