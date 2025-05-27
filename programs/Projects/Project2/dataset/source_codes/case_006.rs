
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct BurnTokenCtxvuto<'info> {
    #[account(mut)] pub burn_account: Account<'info, DataAccount>,
    #[account(mut)] pub owner: Account<'info, DataAccount>,
    #[account(mut)] pub token_program: Account<'info, DataAccount>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAccount {
    pub data: u64,
}

#[program]
pub mod missing_owner_check_006 {
    use super::*;

    pub fn burn_token(ctx: Context<BurnTokenCtxvuto>, amount: u64) -> Result<()> {
        let acct = &mut ctx.accounts.burn_account;
        // custom logic for burn_token
        for _ in 0..amount { acct.data += 1; }
        msg!("Executed burn_token logic");
        Ok(())
    }
}
