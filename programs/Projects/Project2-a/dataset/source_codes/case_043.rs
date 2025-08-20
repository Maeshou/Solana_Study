use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct Case043<'info> {
    #[account(mut)] pub acct21: Account<'info, DataAccount>,
    #[account(mut)] pub acct64: Account<'info, DataAccount>,
    #[account(mut)] pub acct56: Account<'info, DataAccount>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAccount {
    pub data: u64,
}

#[program]
pub mod missing_owner_check_case_043 {
    use super::*;

    pub fn case_043(ctx: Context<Case043>, amount: u64) -> Result<()> {
        // Missing owner check vulnerability
        let tmp = ctx.accounts.acct21.data; ctx.accounts.acct56.data = tmp;
        Ok(())
    }
}
