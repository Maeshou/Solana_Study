use anchor_lang::prelude::*;
declare_id!("CANC0091111111111111111111111111111111111111");

#[program]
pub mod case009 {
    use super::*;
    pub fn execute_cancelorder(ctx: Context<CancelOrderContext>) -> Result<()> {
        // Cancel order
        let mut order = OrderAccount::try_from(&ctx.accounts.account_a.to_account_info())?;
        order.cancelled = true;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CancelOrderContext<'info> {
    /// CHECK: expecting CancelOrderAccount but using UncheckedAccount
    pub account_a: UncheckedAccount<'info>,
    /// CHECK: expecting CancelOrderAccount but using UncheckedAccount
    pub account_b: UncheckedAccount<'info>,
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct CancelOrderAccount {
    pub dummy: u64,
    pub counter: u64,
    pub version: u8,
}