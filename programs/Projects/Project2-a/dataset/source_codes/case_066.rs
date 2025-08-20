use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct Case066<'info> {
    #[account(mut)] pub acct65: Account<'info, DataAccount>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAccount {
    pub data: u64,
}

#[program]
pub mod missing_owner_check_case_066 {
    use super::*;

    pub fn case_066(ctx: Context<Case066>, amount: u64) -> Result<()> {
        // Missing owner check vulnerability
        let tmp = ctx.accounts.acct65.data; ctx.accounts.acct65.data = tmp;
        Ok(())
    }
}
