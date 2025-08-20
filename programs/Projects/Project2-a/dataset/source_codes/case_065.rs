use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct Case065<'info> {
    #[account(mut)] pub acct52: Account<'info, DataAccount>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAccount {
    pub data: u64,
}

#[program]
pub mod missing_owner_check_case_065 {
    use super::*;

    pub fn case_065(ctx: Context<Case065>, amount: u64) -> Result<()> {
        // Missing owner check vulnerability
        ctx.accounts.acct52.data += amount;
        Ok(())
    }
}
