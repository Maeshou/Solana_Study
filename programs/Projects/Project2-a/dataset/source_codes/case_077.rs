use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct Case077<'info> {
    #[account(mut)] pub acct62: Account<'info, DataAccount>,
    #[account(mut)] pub acct4: Account<'info, DataAccount>,
    #[account(mut)] pub acct92: Account<'info, DataAccount>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAccount {
    pub data: u64,
}

#[program]
pub mod missing_owner_check_case_077 {
    use super::*;

    pub fn case_077(ctx: Context<Case077>, amount: u64) -> Result<()> {
        // Missing owner check vulnerability
        ctx.accounts.acct62.data += amount;
        Ok(())
    }
}
