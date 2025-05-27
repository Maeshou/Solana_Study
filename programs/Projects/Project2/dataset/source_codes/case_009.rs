
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct CancelOrderCtxikih<'info> {
    #[account(mut)] pub order_book: Account<'info, DataAccount>,
    #[account(mut)] pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAccount {
    pub data: u64,
}

#[program]
pub mod missing_owner_check_009 {
    use super::*;

    pub fn cancel_order(ctx: Context<CancelOrderCtxikih>, amount: u64) -> Result<()> {
        let acct = &mut ctx.accounts.order_book;
        // custom logic for cancel_order
        let temp = acct.data; acct.data = temp.checked_mul(2).unwrap();
        msg!("Executed cancel_order logic");
        Ok(())
    }
}
