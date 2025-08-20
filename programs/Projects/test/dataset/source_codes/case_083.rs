use anchor_lang::prelude::*;
declare_id!("Case0831111111111111111111111111111111111111");

#[program]
pub mod case083 {
    use super::*;
    pub fn execute_use_credit(ctx: Context<UseCreditContext>) -> Result<()> {
        // Use Case 83: 気候クレジット消費（UseCredit）
        // Vulnerable: using UncheckedAccount where UseCreditAccount is expected
        msg!("Executing execute_use_credit for 気候クレジット消費（UseCredit）");
        // Example logic (dummy operation)
        let mut acct_data = UseCreditAccount::try_from(&ctx.accounts.account_a.to_account_info())?;
        acct_data.dummy = acct_data.dummy.checked_add(1).unwrap();
        Ok(())
    }
}

#[derive(Accounts)]
pub struct UseCreditContext<'info> {
    /// CHECK: expecting UseCreditAccount but using UncheckedAccount
    pub account_a: UncheckedAccount<'info>,
    /// CHECK: expecting UseCreditAccount but using UncheckedAccount
    pub account_b: UncheckedAccount<'info>,
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct UseCreditAccount {
    pub dummy: u64,
}