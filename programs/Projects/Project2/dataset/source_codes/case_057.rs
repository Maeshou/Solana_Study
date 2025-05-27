
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct AddToWhitelistCtxzkuo<'info> {
    #[account(mut)] pub whitelist: Account<'info, DataAccount>,
    #[account(mut)] pub admin: Account<'info, DataAccount>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAccount {
    pub data: u64,
}

#[program]
pub mod missing_owner_check_057 {
    use super::*;

    pub fn add_to_whitelist(ctx: Context<AddToWhitelistCtxzkuo>, amount: u64) -> Result<()> {
        let acct = &mut ctx.accounts.whitelist;
        // custom logic for add_to_whitelist
        acct.data = acct.data.checked_add(amount).unwrap();
        msg!("Executed add_to_whitelist logic");
        Ok(())
    }
}
