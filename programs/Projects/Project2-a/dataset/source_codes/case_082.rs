use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct Case082<'info> {
    #[account(mut)] pub acct57: Account<'info, DataAccount>,
    #[account(mut)] pub acct2: Account<'info, DataAccount>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAccount {
    pub data: u64,
}

#[program]
pub mod missing_owner_check_case_082 {
    use super::*;

    pub fn case_082(ctx: Context<Case082>, amount: u64) -> Result<()> {
        // Missing owner check vulnerability
        ctx.accounts.acct57.data += amount;
        Ok(())
    }
}
