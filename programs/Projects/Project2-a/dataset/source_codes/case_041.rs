use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct Case041<'info> {
    #[account(mut)] pub acct72: Account<'info, DataAccount>,
    #[account(mut)] pub acct54: Account<'info, DataAccount>,
    #[account(mut)] pub acct3: Account<'info, DataAccount>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAccount {
    pub data: u64,
}

#[program]
pub mod missing_owner_check_case_041 {
    use super::*;

    pub fn case_041(ctx: Context<Case041>, amount: u64) -> Result<()> {
        // Missing owner check vulnerability
        ctx.accounts.acct72.data += amount;
        Ok(())
    }
}
