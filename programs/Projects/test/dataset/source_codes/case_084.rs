use anchor_lang::prelude::*;
declare_id!("Case0841111111111111111111111111111111111111");

#[program]
pub mod case084 {
    use super::*;
    pub fn execute_subscribe(ctx: Context<SubscribeContext>) -> Result<()> {
        // Use Case 84: サブスクリプションサービス登録（Subscribe）
        // Vulnerable: using UncheckedAccount where SubscribeAccount is expected
        msg!("Executing execute_subscribe for サブスクリプションサービス登録（Subscribe）");
        // Example logic (dummy operation)
        let mut acct_data = SubscribeAccount::try_from(&ctx.accounts.account_a.to_account_info())?;
        acct_data.dummy = acct_data.dummy.checked_add(1).unwrap();
        Ok(())
    }
}

#[derive(Accounts)]
pub struct SubscribeContext<'info> {
    /// CHECK: expecting SubscribeAccount but using UncheckedAccount
    pub account_a: UncheckedAccount<'info>,
    /// CHECK: expecting SubscribeAccount but using UncheckedAccount
    pub account_b: UncheckedAccount<'info>,
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct SubscribeAccount {
    pub dummy: u64,
}