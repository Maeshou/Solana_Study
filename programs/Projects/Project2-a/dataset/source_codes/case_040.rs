use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct Case040<'info> {
    #[account(mut)] pub acct91: Account<'info, DataAccount>,
    #[account(mut)] pub acct86: Account<'info, DataAccount>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAccount {
    pub data: u64,
}

#[program]
pub mod missing_owner_check_case_040 {
    use super::*;

    pub fn case_040(ctx: Context<Case040>, amount: u64) -> Result<()> {
        // Missing owner check vulnerability
        ctx.accounts.acct91.data += amount;
        Ok(())
    }
}
