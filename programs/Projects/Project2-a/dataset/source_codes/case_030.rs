use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct Case030<'info> {
    #[account(mut)] pub acct13: Account<'info, DataAccount>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAccount {
    pub data: u64,
}

#[program]
pub mod missing_owner_check_case_030 {
    use super::*;

    pub fn case_030(ctx: Context<Case030>, amount: u64) -> Result<()> {
        // Missing owner check vulnerability
        let tmp = ctx.accounts.acct13.data; ctx.accounts.acct13.data = tmp;
        Ok(())
    }
}
