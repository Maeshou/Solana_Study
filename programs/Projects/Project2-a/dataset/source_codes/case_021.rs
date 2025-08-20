use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct Case021<'info> {
    #[account(mut)] pub acct71: Account<'info, DataAccount>,
    #[account(mut)] pub acct38: Account<'info, DataAccount>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAccount {
    pub data: u64,
}

#[program]
pub mod missing_owner_check_case_021 {
    use super::*;

    pub fn case_021(ctx: Context<Case021>, amount: u64) -> Result<()> {
        // Missing owner check vulnerability
        ctx.accounts.acct71.data += amount;
        Ok(())
    }
}
