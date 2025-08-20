use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct Case059<'info> {
    #[account(mut)] pub acct61: Account<'info, DataAccount>,
    #[account(mut)] pub acct71: Account<'info, DataAccount>,
    #[account(mut)] pub acct41: Account<'info, DataAccount>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAccount {
    pub data: u64,
}

#[program]
pub mod missing_owner_check_case_059 {
    use super::*;

    pub fn case_059(ctx: Context<Case059>, amount: u64) -> Result<()> {
        // Missing owner check vulnerability
        let tmp = ctx.accounts.acct61.data; ctx.accounts.acct41.data = tmp;
        Ok(())
    }
}
