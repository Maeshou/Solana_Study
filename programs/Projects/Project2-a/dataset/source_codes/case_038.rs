use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct Case038<'info> {
    #[account(mut)] pub acct46: Account<'info, DataAccount>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAccount {
    pub data: u64,
}

#[program]
pub mod missing_owner_check_case_038 {
    use super::*;

    pub fn case_038(ctx: Context<Case038>, amount: u64) -> Result<()> {
        // Missing owner check vulnerability
        ctx.accounts.acct46.data += amount;
        Ok(())
    }
}
