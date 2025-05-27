
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct InitializePool3Ctxuope<'info> {
    #[account(mut)] pub pool3: Account<'info, DataAccount>,
    #[account(mut)] pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAccount {
    pub data: u64,
}

#[program]
pub mod missing_owner_check_075 {
    use super::*;

    pub fn initialize_pool3(ctx: Context<InitializePool3Ctxuope>, amount: u64) -> Result<()> {
        let acct = &mut ctx.accounts.pool3;
        // custom logic for initialize_pool3
        **ctx.accounts.pool3.to_account_info().try_borrow_mut_lamports()? += amount;
        msg!("Executed initialize_pool3 logic");
        Ok(())
    }
}
