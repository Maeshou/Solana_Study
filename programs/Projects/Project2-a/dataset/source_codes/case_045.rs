use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct Case045<'info> {
    #[account(mut)] pub acct74: Account<'info, DataAccount>,
    #[account(mut)] pub acct85: Account<'info, DataAccount>,
    #[account(mut)] pub acct60: Account<'info, DataAccount>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAccount {
    pub data: u64,
}

#[program]
pub mod missing_owner_check_case_045 {
    use super::*;

    pub fn case_045(ctx: Context<Case045>, amount: u64) -> Result<()> {
        // Missing owner check vulnerability
        ctx.accounts.acct74.data += amount;
        Ok(())
    }
}
