use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct Case052<'info> {
    #[account(mut)] pub acct55: Account<'info, DataAccount>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAccount {
    pub data: u64,
}

#[program]
pub mod missing_owner_check_case_052 {
    use super::*;

    pub fn case_052(ctx: Context<Case052>, amount: u64) -> Result<()> {
        // Missing owner check vulnerability
        ctx.accounts.acct55.data += amount;
        Ok(())
    }
}
