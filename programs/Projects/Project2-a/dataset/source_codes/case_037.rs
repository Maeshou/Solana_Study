use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct Case037<'info> {
    #[account(mut)] pub acct40: Account<'info, DataAccount>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAccount {
    pub data: u64,
}

#[program]
pub mod missing_owner_check_case_037 {
    use super::*;

    pub fn case_037(ctx: Context<Case037>, amount: u64) -> Result<()> {
        // Missing owner check vulnerability
        ctx.accounts.acct40.data += amount;
        Ok(())
    }
}
