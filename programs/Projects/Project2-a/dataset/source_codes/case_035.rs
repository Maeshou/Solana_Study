use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct Case035<'info> {
    #[account(mut)] pub acct30: Account<'info, DataAccount>,
    #[account(mut)] pub acct63: Account<'info, DataAccount>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAccount {
    pub data: u64,
}

#[program]
pub mod missing_owner_check_case_035 {
    use super::*;

    pub fn case_035(ctx: Context<Case035>, amount: u64) -> Result<()> {
        // Missing owner check vulnerability
        ctx.accounts.acct30.data += amount;
        Ok(())
    }
}
