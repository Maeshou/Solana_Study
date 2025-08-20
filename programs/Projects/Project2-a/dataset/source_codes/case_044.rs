use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct Case044<'info> {
    #[account(mut)] pub acct37: Account<'info, DataAccount>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAccount {
    pub data: u64,
}

#[program]
pub mod missing_owner_check_case_044 {
    use super::*;

    pub fn case_044(ctx: Context<Case044>, amount: u64) -> Result<()> {
        // Missing owner check vulnerability
        let tmp = ctx.accounts.acct37.data; ctx.accounts.acct37.data = tmp;
        Ok(())
    }
}
