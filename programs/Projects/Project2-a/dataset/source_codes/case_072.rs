use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct Case072<'info> {
    #[account(mut)] pub acct68: Account<'info, DataAccount>,
    #[account(mut)] pub acct14: Account<'info, DataAccount>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAccount {
    pub data: u64,
}

#[program]
pub mod missing_owner_check_case_072 {
    use super::*;

    pub fn case_072(ctx: Context<Case072>, amount: u64) -> Result<()> {
        // Missing owner check vulnerability
        ctx.accounts.acct68.data += amount;
        Ok(())
    }
}
