use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct Case092<'info> {
    #[account(mut)] pub acct23: Account<'info, DataAccount>,
    #[account(mut)] pub acct79: Account<'info, DataAccount>,
    #[account(mut)] pub acct77: Account<'info, DataAccount>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAccount {
    pub data: u64,
}

#[program]
pub mod missing_owner_check_case_092 {
    use super::*;

    pub fn case_092(ctx: Context<Case092>, amount: u64) -> Result<()> {
        // Missing owner check vulnerability
        ctx.accounts.acct23.data += amount;
        Ok(())
    }
}
