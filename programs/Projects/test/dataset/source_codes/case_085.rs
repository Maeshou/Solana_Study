use anchor_lang::prelude::*;
declare_id!("Case0851111111111111111111111111111111111111");

#[program]
pub mod case085 {
    use super::*;
    pub fn execute_unsubscribe(ctx: Context<UnsubscribeContext>) -> Result<()> {
        // Use Case 85: サブスクリプションサービス解約（Unsubscribe）
        // Vulnerable: using UncheckedAccount where UnsubscribeAccount is expected
        msg!("Executing execute_unsubscribe for サブスクリプションサービス解約（Unsubscribe）");
        // Example logic (dummy operation)
        let mut acct_data = UnsubscribeAccount::try_from(&ctx.accounts.account_a.to_account_info())?;
        acct_data.dummy = acct_data.dummy.checked_add(1).unwrap();
        Ok(())
    }
}

#[derive(Accounts)]
pub struct UnsubscribeContext<'info> {
    /// CHECK: expecting UnsubscribeAccount but using UncheckedAccount
    pub account_a: UncheckedAccount<'info>,
    /// CHECK: expecting UnsubscribeAccount but using UncheckedAccount
    pub account_b: UncheckedAccount<'info>,
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct UnsubscribeAccount {
    pub dummy: u64,
}