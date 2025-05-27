
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct SettleCtxikig<'info> {
    #[account(mut)] pub vault: Account<'info, DataAccount>,
    #[account(mut)] pub settler: Account<'info, DataAccount>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAccount {
    pub data: u64,
}

#[program]
pub mod missing_owner_check_010 {
    use super::*;

    pub fn settle(ctx: Context<SettleCtxikig>, amount: u64) -> Result<()> {
        let acct = &mut ctx.accounts.vault;
        // custom logic for settle
        **ctx.accounts.vault.to_account_info().try_borrow_mut_lamports()? += amount;
        msg!("Executed settle logic");
        Ok(())
    }
}
