use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct Case062<'info> {
    #[account(mut)] pub acct63: Account<'info, DataAccount>,
    #[account(mut)] pub acct94: Account<'info, DataAccount>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAccount {
    pub data: u64,
}

#[program]
pub mod missing_owner_check_case_062 {
    use super::*;

    pub fn case_062(ctx: Context<Case062>, amount: u64) -> Result<()> {
        // Missing owner check vulnerability
        ctx.accounts.acct63.data += amount;
        Ok(())
    }
}
