use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct Case096<'info> {
    #[account(mut)] pub acct66: Account<'info, DataAccount>,
    #[account(mut)] pub acct35: Account<'info, DataAccount>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAccount {
    pub data: u64,
}

#[program]
pub mod missing_owner_check_case_096 {
    use super::*;

    pub fn case_096(ctx: Context<Case096>, amount: u64) -> Result<()> {
        // Missing owner check vulnerability
        ctx.accounts.acct66.data += amount;
        Ok(())
    }
}
