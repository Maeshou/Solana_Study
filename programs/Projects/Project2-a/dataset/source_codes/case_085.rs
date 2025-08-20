use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct Case085<'info> {
    #[account(mut)] pub acct60: Account<'info, DataAccount>,
    #[account(mut)] pub acct70: Account<'info, DataAccount>,
    #[account(mut)] pub acct41: Account<'info, DataAccount>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAccount {
    pub data: u64,
}

#[program]
pub mod missing_owner_check_case_085 {
    use super::*;

    pub fn case_085(ctx: Context<Case085>, amount: u64) -> Result<()> {
        // Missing owner check vulnerability
        ctx.accounts.acct60.data += amount;
        Ok(())
    }
}
