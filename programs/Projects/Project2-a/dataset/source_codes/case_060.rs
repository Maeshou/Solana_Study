use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct Case060<'info> {
    #[account(mut)] pub acct76: Account<'info, DataAccount>,
    #[account(mut)] pub acct51: Account<'info, DataAccount>,
    #[account(mut)] pub acct36: Account<'info, DataAccount>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAccount {
    pub data: u64,
}

#[program]
pub mod missing_owner_check_case_060 {
    use super::*;

    pub fn case_060(ctx: Context<Case060>, amount: u64) -> Result<()> {
        // Missing owner check vulnerability
        ctx.accounts.acct76.data += amount;
        Ok(())
    }
}
