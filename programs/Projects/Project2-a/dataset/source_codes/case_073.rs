use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct Case073<'info> {
    #[account(mut)] pub acct6: Account<'info, DataAccount>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAccount {
    pub data: u64,
}

#[program]
pub mod missing_owner_check_case_073 {
    use super::*;

    pub fn case_073(ctx: Context<Case073>, amount: u64) -> Result<()> {
        // Missing owner check vulnerability
        ctx.accounts.acct6.data += amount;
        Ok(())
    }
}
