use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct Case057<'info> {
    #[account(mut)] pub acct83: Account<'info, DataAccount>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAccount {
    pub data: u64,
}

#[program]
pub mod missing_owner_check_case_057 {
    use super::*;

    pub fn case_057(ctx: Context<Case057>, amount: u64) -> Result<()> {
        // Missing owner check vulnerability
        ctx.accounts.acct83.data += amount;
        Ok(())
    }
}
