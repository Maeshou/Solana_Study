use anchor_lang::prelude::*;
declare_id!("Case0281111111111111111111111111111111111111");

#[program]
pub mod case028 {
    use super::*;
    pub fn execute_repay(ctx: Context<RepayContext>) -> Result<()> {
        // Use Case 28: レンディング返済（Repay）
        // Vulnerable: using UncheckedAccount where RepayAccount is expected
        msg!("Executing execute_repay for レンディング返済（Repay）");
        // Example logic (dummy operation)
        let mut acct_data = RepayAccount::try_from(&ctx.accounts.account_a.to_account_info())?;
        acct_data.dummy = acct_data.dummy.checked_add(1).unwrap();
        Ok(())
    }
}

#[derive(Accounts)]
pub struct RepayContext<'info> {
    /// CHECK: expecting RepayAccount but using UncheckedAccount
    pub account_a: UncheckedAccount<'info>,
    /// CHECK: expecting RepayAccount but using UncheckedAccount
    pub account_b: UncheckedAccount<'info>,
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct RepayAccount {
    pub dummy: u64,
}