use anchor_lang::prelude::*;
declare_id!("Case0081111111111111111111111111111111111111");

#[program]
pub mod case008 {
    use super::*;
    pub fn execute_place_order(ctx: Context<PlaceOrderContext>) -> Result<()> {
        // Use Case 8: DEX のオーダー作成（PlaceOrder）
        // Vulnerable: using UncheckedAccount where PlaceOrderAccount is expected
        msg!("Executing execute_place_order for DEX のオーダー作成（PlaceOrder）");
        // Example logic (dummy operation)
        let mut acct_data = PlaceOrderAccount::try_from(&ctx.accounts.account_a.to_account_info())?;
        acct_data.dummy = acct_data.dummy.checked_add(1).unwrap();
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
}