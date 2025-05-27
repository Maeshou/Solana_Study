
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct LockAssetCtxhido<'info> {
    #[account(mut)] pub bridge: Account<'info, DataAccount>,
    #[account(mut)] pub locker: Account<'info, DataAccount>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAccount {
    pub data: u64,
}

#[program]
pub mod missing_owner_check_080 {
    use super::*;

    pub fn lock_asset(ctx: Context<LockAssetCtxhido>, amount: u64) -> Result<()> {
        let acct = &mut ctx.accounts.bridge;
        // custom logic for lock_asset
        acct.data = acct.data.checked_add(amount).unwrap();
        msg!("Executed lock_asset logic");
        Ok(())
    }
}
