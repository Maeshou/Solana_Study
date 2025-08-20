use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct Case055<'info> {
    #[account(mut)] pub acct5: Account<'info, DataAccount>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAccount {
    pub data: u64,
}

#[program]
pub mod missing_owner_check_case_055 {
    use super::*;

    pub fn case_055(ctx: Context<Case055>, amount: u64) -> Result<()> {
        // Missing owner check vulnerability
        ctx.accounts.acct5.data += amount;
        Ok(())
    }
}
