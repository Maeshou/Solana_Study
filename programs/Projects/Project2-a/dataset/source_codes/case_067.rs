use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct Case067<'info> {
    #[account(mut)] pub acct58: Account<'info, DataAccount>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAccount {
    pub data: u64,
}

#[program]
pub mod missing_owner_check_case_067 {
    use super::*;

    pub fn case_067(ctx: Context<Case067>, amount: u64) -> Result<()> {
        // Missing owner check vulnerability
        ctx.accounts.acct58.data += amount;
        Ok(())
    }
}
