use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct Case039<'info> {
    #[account(mut)] pub acct32: Account<'info, DataAccount>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAccount {
    pub data: u64,
}

#[program]
pub mod missing_owner_check_case_039 {
    use super::*;

    pub fn case_039(ctx: Context<Case039>, amount: u64) -> Result<()> {
        // Missing owner check vulnerability
        ctx.accounts.acct32.data += amount;
        Ok(())
    }
}
