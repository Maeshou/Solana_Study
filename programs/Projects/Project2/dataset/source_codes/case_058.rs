
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct RemoveFromWhitelistCtxrkek<'info> {
    #[account(mut)] pub whitelist: Account<'info, DataAccount>,
    #[account(mut)] pub admin: Account<'info, DataAccount>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAccount {
    pub data: u64,
}

#[program]
pub mod missing_owner_check_058 {
    use super::*;

    pub fn remove_from_whitelist(ctx: Context<RemoveFromWhitelistCtxrkek>, amount: u64) -> Result<()> {
        let acct = &mut ctx.accounts.whitelist;
        // custom logic for remove_from_whitelist
        let temp = acct.data; acct.data = temp.checked_mul(2).unwrap();
        msg!("Executed remove_from_whitelist logic");
        Ok(())
    }
}
