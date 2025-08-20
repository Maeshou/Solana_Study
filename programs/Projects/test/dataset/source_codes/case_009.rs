use anchor_lang::prelude::*;
declare_id!("Case0091111111111111111111111111111111111111");

#[program]
pub mod case009 {
    use super::*;
    pub fn execute_cancel_order(ctx: Context<CancelOrderContext>) -> Result<()> {
        // Use Case 9: DEX のオーダーキャンセル（CancelOrder）
        // Vulnerable: using UncheckedAccount where CancelOrderAccount is expected
        msg!("Executing execute_cancel_order for DEX のオーダーキャンセル（CancelOrder）");
        // Example logic (dummy operation)
        let mut acct_data = CancelOrderAccount::try_from(&ctx.accounts.account_a.to_account_info())?;
        acct_data.dummy = acct_data.dummy.checked_add(1).unwrap();
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
}