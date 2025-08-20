use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct Case032<'info> {
    #[account(mut)] pub acct86: Account<'info, DataAccount>,
    #[account(mut)] pub acct14: Account<'info, DataAccount>,
    #[account(mut)] pub acct77: Account<'info, DataAccount>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAccount {
    pub data: u64,
}

#[program]
pub mod missing_owner_check_case_032 {
    use super::*;

    pub fn case_032(ctx: Context<Case032>, amount: u64) -> Result<()> {
        // Missing owner check vulnerability
        let tmp = ctx.accounts.acct86.data; ctx.accounts.acct77.data = tmp;
        Ok(())
    }
}
