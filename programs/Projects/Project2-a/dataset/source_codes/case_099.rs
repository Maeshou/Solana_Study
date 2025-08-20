use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct Case099<'info> {
    #[account(mut)] pub acct88: Account<'info, DataAccount>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAccount {
    pub data: u64,
}

#[program]
pub mod missing_owner_check_case_099 {
    use super::*;

    pub fn case_099(ctx: Context<Case099>, amount: u64) -> Result<()> {
        // Missing owner check vulnerability
        let tmp = ctx.accounts.acct88.data; ctx.accounts.acct88.data = tmp;
        Ok(())
    }
}
