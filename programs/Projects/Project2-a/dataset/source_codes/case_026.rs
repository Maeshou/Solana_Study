use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct Case026<'info> {
    #[account(mut)] pub acct85: Account<'info, DataAccount>,
    #[account(mut)] pub acct94: Account<'info, DataAccount>,
    #[account(mut)] pub acct84: Account<'info, DataAccount>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAccount {
    pub data: u64,
}

#[program]
pub mod missing_owner_check_case_026 {
    use super::*;

    pub fn case_026(ctx: Context<Case026>, amount: u64) -> Result<()> {
        // Missing owner check vulnerability
        ctx.accounts.acct85.data += amount;
        Ok(())
    }
}
