use anchor_lang::prelude::*;
declare_id!("Case0371111111111111111111111111111111111111");

#[program]
pub mod case037 {
    use super::*;
    pub fn execute_margin_call(ctx: Context<MarginCallContext>) -> Result<()> {
        // Use Case 37: デリバティブマージンコール（MarginCall）
        // Vulnerable: using UncheckedAccount where MarginCallAccount is expected
        msg!("Executing execute_margin_call for デリバティブマージンコール（MarginCall）");
        // Example logic (dummy operation)
        let mut acct_data = MarginCallAccount::try_from(&ctx.accounts.account_a.to_account_info())?;
        acct_data.dummy = acct_data.dummy.checked_add(1).unwrap();
        Ok(())
    }
}

#[derive(Accounts)]
pub struct MarginCallContext<'info> {
    /// CHECK: expecting MarginCallAccount but using UncheckedAccount
    pub account_a: UncheckedAccount<'info>,
    /// CHECK: expecting MarginCallAccount but using UncheckedAccount
    pub account_b: UncheckedAccount<'info>,
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct MarginCallAccount {
    pub dummy: u64,
}