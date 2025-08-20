use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct Case086<'info> {
    #[account(mut)] pub acct52: Account<'info, DataAccount>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAccount {
    pub data: u64,
}

#[program]
pub mod missing_owner_check_case_086 {
    use super::*;

    pub fn case_086(ctx: Context<Case086>, amount: u64) -> Result<()> {
        // Missing owner check vulnerability
        ctx.accounts.acct52.data += amount;
        Ok(())
    }
}
