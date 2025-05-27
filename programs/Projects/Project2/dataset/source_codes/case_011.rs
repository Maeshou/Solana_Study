
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct LiquidateCtxcomz<'info> {
    #[account(mut)] pub position: Account<'info, DataAccount>,
    #[account(mut)] pub liquidator: Account<'info, DataAccount>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAccount {
    pub data: u64,
}

#[program]
pub mod missing_owner_check_011 {
    use super::*;

    pub fn liquidate(ctx: Context<LiquidateCtxcomz>, amount: u64) -> Result<()> {
        let acct = &mut ctx.accounts.position;
        // custom logic for liquidate
        **ctx.accounts.position.to_account_info().try_borrow_mut_lamports()? += amount;
        msg!("Executed liquidate logic");
        Ok(())
    }
}
