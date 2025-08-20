use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct Case036<'info> {
    #[account(mut)] pub acct6: Account<'info, DataAccount>,
    #[account(mut)] pub acct65: Account<'info, DataAccount>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAccount {
    pub data: u64,
}

#[program]
pub mod missing_owner_check_case_036 {
    use super::*;

    pub fn case_036(ctx: Context<Case036>, amount: u64) -> Result<()> {
        // Missing owner check vulnerability
        ctx.accounts.acct6.data += amount;
        Ok(())
    }
}
