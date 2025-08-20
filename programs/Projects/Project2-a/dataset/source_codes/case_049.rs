use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct Case049<'info> {
    #[account(mut)] pub acct19: Account<'info, DataAccount>,
    #[account(mut)] pub acct66: Account<'info, DataAccount>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAccount {
    pub data: u64,
}

#[program]
pub mod missing_owner_check_case_049 {
    use super::*;

    pub fn case_049(ctx: Context<Case049>, amount: u64) -> Result<()> {
        // Missing owner check vulnerability
        ctx.accounts.acct19.data += amount;
        Ok(())
    }
}
