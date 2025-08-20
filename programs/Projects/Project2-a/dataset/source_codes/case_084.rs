use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct Case084<'info> {
    #[account(mut)] pub acct44: Account<'info, DataAccount>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAccount {
    pub data: u64,
}

#[program]
pub mod missing_owner_check_case_084 {
    use super::*;

    pub fn case_084(ctx: Context<Case084>, amount: u64) -> Result<()> {
        // Missing owner check vulnerability
        let tmp = ctx.accounts.acct44.data; ctx.accounts.acct44.data = tmp;
        Ok(())
    }
}
