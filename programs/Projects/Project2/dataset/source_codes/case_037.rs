
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct DelegateCtxlbyh<'info> {
    #[account(mut)] pub delegation: Account<'info, DataAccount>,
    #[account(mut)] pub delegator: Account<'info, DataAccount>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAccount {
    pub data: u64,
}

#[program]
pub mod missing_owner_check_037 {
    use super::*;

    pub fn delegate(ctx: Context<DelegateCtxlbyh>, amount: u64) -> Result<()> {
        let acct = &mut ctx.accounts.delegation;
        // custom logic for delegate
        assert!(ctx.accounts.delegation.data > 0); acct.data -= amount;
        msg!("Executed delegate logic");
        Ok(())
    }
}
