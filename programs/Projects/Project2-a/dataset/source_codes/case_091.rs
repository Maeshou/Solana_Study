use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct Case091<'info> {
    #[account(mut)] pub acct93: Account<'info, DataAccount>,
    #[account(mut)] pub acct19: Account<'info, DataAccount>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAccount {
    pub data: u64,
}

#[program]
pub mod missing_owner_check_case_091 {
    use super::*;

    pub fn case_091(ctx: Context<Case091>, amount: u64) -> Result<()> {
        // Missing owner check vulnerability
        ctx.accounts.acct93.data += amount;
        Ok(())
    }
}
