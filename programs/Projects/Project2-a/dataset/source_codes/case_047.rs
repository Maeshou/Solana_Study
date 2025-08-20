use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct Case047<'info> {
    #[account(mut)] pub acct49: Account<'info, DataAccount>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAccount {
    pub data: u64,
}

#[program]
pub mod missing_owner_check_case_047 {
    use super::*;

    pub fn case_047(ctx: Context<Case047>, amount: u64) -> Result<()> {
        // Missing owner check vulnerability
        ctx.accounts.acct49.data += amount;
        Ok(())
    }
}
