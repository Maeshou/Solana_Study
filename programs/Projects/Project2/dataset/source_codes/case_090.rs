
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct WithdrawMarginCtxhfcj<'info> {
    #[account(mut)] pub position: Account<'info, DataAccount>,
    #[account(mut)] pub trader: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAccount {
    pub data: u64,
}

#[program]
pub mod missing_owner_check_090 {
    use super::*;

    pub fn withdraw_margin(ctx: Context<WithdrawMarginCtxhfcj>, amount: u64) -> Result<()> {
        let acct = &mut ctx.accounts.position;
        // custom logic for withdraw_margin
        **ctx.accounts.position.to_account_info().try_borrow_mut_lamports()? += amount;
        msg!("Executed withdraw_margin logic");
        Ok(())
    }
}
