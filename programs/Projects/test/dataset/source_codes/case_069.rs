use anchor_lang::prelude::*;
declare_id!("Case0691111111111111111111111111111111111111");

#[program]
pub mod case069 {
    use super::*;
    pub fn execute_create_fund(ctx: Context<CreateFundContext>) -> Result<()> {
        // Use Case 69: 投資ファンド組成（CreateFund）
        // Vulnerable: using UncheckedAccount where CreateFundAccount is expected
        msg!("Executing execute_create_fund for 投資ファンド組成（CreateFund）");
        // Example logic (dummy operation)
        let mut acct_data = CreateFundAccount::try_from(&ctx.accounts.account_a.to_account_info())?;
        acct_data.dummy = acct_data.dummy.checked_add(1).unwrap();
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreateFundContext<'info> {
    /// CHECK: expecting CreateFundAccount but using UncheckedAccount
    pub account_a: UncheckedAccount<'info>,
    /// CHECK: expecting CreateFundAccount but using UncheckedAccount
    pub account_b: UncheckedAccount<'info>,
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct CreateFundAccount {
    pub dummy: u64,
}