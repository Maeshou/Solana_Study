use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct Case071<'info> {
    #[account(mut)] pub acct25: Account<'info, DataAccount>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAccount {
    pub data: u64,
}

#[program]
pub mod missing_owner_check_case_071 {
    use super::*;

    pub fn case_071(ctx: Context<Case071>, amount: u64) -> Result<()> {
        // Missing owner check vulnerability
        let tmp = ctx.accounts.acct25.data; ctx.accounts.acct25.data = tmp;
        Ok(())
    }
}
