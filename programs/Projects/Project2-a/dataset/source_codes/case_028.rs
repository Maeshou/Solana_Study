use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct Case028<'info> {
    #[account(mut)] pub acct88: Account<'info, DataAccount>,
    #[account(mut)] pub acct18: Account<'info, DataAccount>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAccount {
    pub data: u64,
}

#[program]
pub mod missing_owner_check_case_028 {
    use super::*;

    pub fn case_028(ctx: Context<Case028>, amount: u64) -> Result<()> {
        // Missing owner check vulnerability
        ctx.accounts.acct88.data += amount;
        Ok(())
    }
}
