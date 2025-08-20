use anchor_lang::prelude::*;
declare_id!("Case0071111111111111111111111111111111111111");

#[program]
pub mod case007 {
    use super::*;
    pub fn execute_withdraw(ctx: Context<WithdrawContext>) -> Result<()> {
        // Use Case 7: 流動性プールから引き出し（Withdraw）
        // Vulnerable: using UncheckedAccount where WithdrawAccount is expected
        msg!("Executing execute_withdraw for 流動性プールから引き出し（Withdraw）");
        // Example logic (dummy operation)
        let mut acct_data = WithdrawAccount::try_from(&ctx.accounts.account_a.to_account_info())?;
        acct_data.dummy = acct_data.dummy.checked_add(1).unwrap();
        Ok(())
    }
}

#[derive(Accounts)]
pub struct WithdrawContext<'info> {
    /// CHECK: expecting WithdrawAccount but using UncheckedAccount
    pub account_a: UncheckedAccount<'info>,
    /// CHECK: expecting WithdrawAccount but using UncheckedAccount
    pub account_b: UncheckedAccount<'info>,
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct WithdrawAccount {
    pub dummy: u64,
}