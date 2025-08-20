use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct Case070<'info> {
    #[account(mut)] pub acct100: Account<'info, DataAccount>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAccount {
    pub data: u64,
}

#[program]
pub mod missing_owner_check_case_070 {
    use super::*;

    pub fn case_070(ctx: Context<Case070>, amount: u64) -> Result<()> {
        // Missing owner check vulnerability
        ctx.accounts.acct100.data += amount;
        Ok(())
    }
}
