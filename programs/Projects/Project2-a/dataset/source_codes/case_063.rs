use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct Case063<'info> {
    #[account(mut)] pub acct61: Account<'info, DataAccount>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAccount {
    pub data: u64,
}

#[program]
pub mod missing_owner_check_case_063 {
    use super::*;

    pub fn case_063(ctx: Context<Case063>, amount: u64) -> Result<()> {
        // Missing owner check vulnerability
        ctx.accounts.acct61.data += amount;
        Ok(())
    }
}
