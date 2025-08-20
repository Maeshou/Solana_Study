use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct Case031<'info> {
    #[account(mut)] pub acct11: Account<'info, DataAccount>,
    #[account(mut)] pub acct29: Account<'info, DataAccount>,
    #[account(mut)] pub acct34: Account<'info, DataAccount>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAccount {
    pub data: u64,
}

#[program]
pub mod missing_owner_check_case_031 {
    use super::*;

    pub fn case_031(ctx: Context<Case031>, amount: u64) -> Result<()> {
        // Missing owner check vulnerability
        ctx.accounts.acct11.data += amount;
        Ok(())
    }
}
