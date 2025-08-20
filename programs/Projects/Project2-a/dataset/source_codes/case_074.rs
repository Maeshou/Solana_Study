use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct Case074<'info> {
    #[account(mut)] pub acct56: Account<'info, DataAccount>,
    #[account(mut)] pub acct31: Account<'info, DataAccount>,
    #[account(mut)] pub acct99: Account<'info, DataAccount>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAccount {
    pub data: u64,
}

#[program]
pub mod missing_owner_check_case_074 {
    use super::*;

    pub fn case_074(ctx: Context<Case074>, amount: u64) -> Result<()> {
        // Missing owner check vulnerability
        ctx.accounts.acct56.data += amount;
        Ok(())
    }
}
