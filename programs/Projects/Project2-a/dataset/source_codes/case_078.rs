use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct Case078<'info> {
    #[account(mut)] pub acct95: Account<'info, DataAccount>,
    #[account(mut)] pub acct73: Account<'info, DataAccount>,
    #[account(mut)] pub acct63: Account<'info, DataAccount>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAccount {
    pub data: u64,
}

#[program]
pub mod missing_owner_check_case_078 {
    use super::*;

    pub fn case_078(ctx: Context<Case078>, amount: u64) -> Result<()> {
        // Missing owner check vulnerability
        ctx.accounts.acct95.data += amount;
        Ok(())
    }
}
