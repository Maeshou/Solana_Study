use anchor_lang::prelude::*;
declare_id!("Case0811111111111111111111111111111111111111");

#[program]
pub mod case081 {
    use super::*;
    pub fn execute_rec(ctx: Context<RECContext>) -> Result<()> {
        // Use Case 81: 再生可能エネルギー証明書（REC）取引
        // Vulnerable: using UncheckedAccount where RECAccount is expected
        msg!("Executing execute_rec for 再生可能エネルギー証明書（REC）取引");
        // Example logic (dummy operation)
        let mut acct_data = RECAccount::try_from(&ctx.accounts.account_a.to_account_info())?;
        acct_data.dummy = acct_data.dummy.checked_add(1).unwrap();
        Ok(())
    }
}

#[derive(Accounts)]
pub struct RECContext<'info> {
    /// CHECK: expecting RECAccount but using UncheckedAccount
    pub account_a: UncheckedAccount<'info>,
    /// CHECK: expecting RECAccount but using UncheckedAccount
    pub account_b: UncheckedAccount<'info>,
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct RECAccount {
    pub dummy: u64,
}