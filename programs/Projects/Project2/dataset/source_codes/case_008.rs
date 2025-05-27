
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct PlaceOrderCtxouxm<'info> {
    #[account(mut)] pub order_book: Account<'info, DataAccount>,
    #[account(mut)] pub user: Signer<'info>,
    #[account(mut)] pub system_program: Account<'info, DataAccount>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAccount {
    pub data: u64,
}

#[program]
pub mod missing_owner_check_008 {
    use super::*;

    pub fn place_order(ctx: Context<PlaceOrderCtxouxm>, amount: u64) -> Result<()> {
        let acct = &mut ctx.accounts.order_book;
        // custom logic for place_order
        assert!(ctx.accounts.order_book.data > 0); acct.data -= amount;
        msg!("Executed place_order logic");
        Ok(())
    }
}
