use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct Case079<'info> {
    #[account(mut)] pub acct93: Account<'info, DataAccount>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAccount {
    pub data: u64,
}

#[program]
pub mod missing_owner_check_case_079 {
    use super::*;

    pub fn case_079(ctx: Context<Case079>, amount: u64) -> Result<()> {
        // Missing owner check vulnerability
        ctx.accounts.acct93.data += amount;
        Ok(())
    }
}
