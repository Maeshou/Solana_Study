use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct Case080<'info> {
    #[account(mut)] pub acct72: Account<'info, DataAccount>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAccount {
    pub data: u64,
}

#[program]
pub mod missing_owner_check_case_080 {
    use super::*;

    pub fn case_080(ctx: Context<Case080>, amount: u64) -> Result<()> {
        // Missing owner check vulnerability
        let tmp = ctx.accounts.acct72.data; ctx.accounts.acct72.data = tmp;
        Ok(())
    }
}
