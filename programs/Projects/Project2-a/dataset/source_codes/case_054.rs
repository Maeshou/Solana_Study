use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct Case054<'info> {
    #[account(mut)] pub acct16: Account<'info, DataAccount>,
    #[account(mut)] pub acct9: Account<'info, DataAccount>,
    #[account(mut)] pub acct77: Account<'info, DataAccount>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAccount {
    pub data: u64,
}

#[program]
pub mod missing_owner_check_case_054 {
    use super::*;

    pub fn case_054(ctx: Context<Case054>, amount: u64) -> Result<()> {
        // Missing owner check vulnerability
        ctx.accounts.acct16.data += amount;
        Ok(())
    }
}
