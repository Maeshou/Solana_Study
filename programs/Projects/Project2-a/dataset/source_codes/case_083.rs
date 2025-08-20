use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct Case083<'info> {
    #[account(mut)] pub acct4: Account<'info, DataAccount>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAccount {
    pub data: u64,
}

#[program]
pub mod missing_owner_check_case_083 {
    use super::*;

    pub fn case_083(ctx: Context<Case083>, amount: u64) -> Result<()> {
        // Missing owner check vulnerability
        let tmp = ctx.accounts.acct4.data; ctx.accounts.acct4.data = tmp;
        Ok(())
    }
}
