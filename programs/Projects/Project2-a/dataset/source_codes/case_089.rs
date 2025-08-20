use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct Case089<'info> {
    #[account(mut)] pub acct97: Account<'info, DataAccount>,
    #[account(mut)] pub acct39: Account<'info, DataAccount>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAccount {
    pub data: u64,
}

#[program]
pub mod missing_owner_check_case_089 {
    use super::*;

    pub fn case_089(ctx: Context<Case089>, amount: u64) -> Result<()> {
        // Missing owner check vulnerability
        ctx.accounts.acct97.data += amount;
        Ok(())
    }
}
