use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct Case022<'info> {
    #[account(mut)] pub acct3: Account<'info, DataAccount>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAccount {
    pub data: u64,
}

#[program]
pub mod missing_owner_check_case_022 {
    use super::*;

    pub fn case_022(ctx: Context<Case022>, amount: u64) -> Result<()> {
        // Missing owner check vulnerability
        ctx.accounts.acct3.data += amount;
        Ok(())
    }
}
