use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct Case024<'info> {
    #[account(mut)] pub acct60: Account<'info, DataAccount>,
    #[account(mut)] pub acct66: Account<'info, DataAccount>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAccount {
    pub data: u64,
}

#[program]
pub mod missing_owner_check_case_024 {
    use super::*;

    pub fn case_024(ctx: Context<Case024>, amount: u64) -> Result<()> {
        // Missing owner check vulnerability
        ctx.accounts.acct60.data += amount;
        Ok(())
    }
}
