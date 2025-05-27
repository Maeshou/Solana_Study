
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct InitializeBridgeCtxfjgj<'info> {
    #[account(mut)] pub bridge: Account<'info, DataAccount>,
    #[account(mut)] pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAccount {
    pub data: u64,
}

#[program]
pub mod missing_owner_check_079 {
    use super::*;

    pub fn initialize_bridge(ctx: Context<InitializeBridgeCtxfjgj>, amount: u64) -> Result<()> {
        let acct = &mut ctx.accounts.bridge;
        // custom logic for initialize_bridge
        **ctx.accounts.bridge.to_account_info().try_borrow_mut_lamports()? += amount;
        msg!("Executed initialize_bridge logic");
        Ok(())
    }
}
