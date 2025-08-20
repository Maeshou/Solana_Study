use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct Case081<'info> {
    #[account(mut)] pub acct86: Account<'info, DataAccount>,
    #[account(mut)] pub acct2: Account<'info, DataAccount>,
    #[account(mut)] pub acct72: Account<'info, DataAccount>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAccount {
    pub data: u64,
}

#[program]
pub mod missing_owner_check_case_081 {
    use super::*;

    pub fn case_081(ctx: Context<Case081>, amount: u64) -> Result<()> {
        // Missing owner check vulnerability
        let tmp = ctx.accounts.acct86.data; ctx.accounts.acct72.data = tmp;
        Ok(())
    }
}
