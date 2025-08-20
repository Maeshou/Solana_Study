use anchor_lang::prelude::*;
declare_id!("Case0061111111111111111111111111111111111111");

#[program]
pub mod case006 {
    use super::*;
    pub fn execute_deposit(ctx: Context<DepositContext>) -> Result<()> {
        // Use Case 6: 流動性プールに預け入れ（Deposit）
        // Vulnerable: using UncheckedAccount where DepositAccount is expected
        msg!("Executing execute_deposit for 流動性プールに預け入れ（Deposit）");
        // Example logic (dummy operation)
        let mut acct_data = DepositAccount::try_from(&ctx.accounts.account_a.to_account_info())?;
        acct_data.dummy = acct_data.dummy.checked_add(1).unwrap();
        Ok(())
    }
}

#[derive(Accounts)]
pub struct DepositContext<'info> {
    /// CHECK: expecting DepositAccount but using UncheckedAccount
    pub account_a: UncheckedAccount<'info>,
    /// CHECK: expecting DepositAccount but using UncheckedAccount
    pub account_b: UncheckedAccount<'info>,
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DepositAccount {
    pub dummy: u64,
}