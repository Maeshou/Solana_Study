use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct Case027<'info> {
    #[account(mut)] pub acct87: Account<'info, DataAccount>,
    #[account(mut)] pub acct7: Account<'info, DataAccount>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAccount {
    pub data: u64,
}

#[program]
pub mod missing_owner_check_case_027 {
    use super::*;

    pub fn case_027(ctx: Context<Case027>, amount: u64) -> Result<()> {
        // Missing owner check vulnerability
        ctx.accounts.acct87.data += amount;
        Ok(())
    }
}
