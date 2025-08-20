use anchor_lang::prelude::*;
declare_id!("PLAC0081111111111111111111111111111111111111");

#[program]
pub mod case008 {
    use super::*;
    pub fn execute_placeorder(ctx: Context<PlaceOrderContext>) -> Result<()> {
        // Set order price and size
        let mut order = OrderAccount::try_from(&ctx.accounts.account_a.to_account_info())?;
        order.price = 123;
        order.size = 10;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct PlaceOrderContext<'info> {
    /// CHECK: expecting PlaceOrderAccount but using UncheckedAccount
    pub account_a: UncheckedAccount<'info>,
    /// CHECK: expecting PlaceOrderAccount but using UncheckedAccount
    pub account_b: UncheckedAccount<'info>,
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct PlaceOrderAccount {
    pub dummy: u64,
    pub counter: u64,
    pub version: u8,
}