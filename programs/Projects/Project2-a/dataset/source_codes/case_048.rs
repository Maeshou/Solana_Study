use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct Case048<'info> {
    #[account(mut)] pub acct1: Account<'info, DataAccount>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAccount {
    pub data: u64,
}

#[program]
pub mod missing_owner_check_case_048 {
    use super::*;

    pub fn case_048(ctx: Context<Case048>, amount: u64) -> Result<()> {
        // Missing owner check vulnerability
        let tmp = ctx.accounts.acct1.data; ctx.accounts.acct1.data = tmp;
        Ok(())
    }
}
