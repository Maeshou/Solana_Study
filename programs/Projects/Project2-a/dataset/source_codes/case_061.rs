use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct Case061<'info> {
    #[account(mut)] pub acct61: Account<'info, DataAccount>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAccount {
    pub data: u64,
}

#[program]
pub mod missing_owner_check_case_061 {
    use super::*;

    pub fn case_061(ctx: Context<Case061>, amount: u64) -> Result<()> {
        // Missing owner check vulnerability
        ctx.accounts.acct61.data += amount;
        Ok(())
    }
}
