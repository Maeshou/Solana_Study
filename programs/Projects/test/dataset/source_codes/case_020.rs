use anchor_lang::prelude::*;
declare_id!("Case0201111111111111111111111111111111111111");

#[program]
pub mod case020 {
    use super::*;
    pub fn execute_execute_multi_sig(ctx: Context<ExecuteMultiSigContext>) -> Result<()> {
        // Use Case 20: マルチシグ署名実行（ExecuteMultiSig）
        // Vulnerable: using UncheckedAccount where ExecuteMultiSigAccount is expected
        msg!("Executing execute_execute_multi_sig for マルチシグ署名実行（ExecuteMultiSig）");
        // Example logic (dummy operation)
        let mut acct_data = ExecuteMultiSigAccount::try_from(&ctx.accounts.account_a.to_account_info())?;
        acct_data.dummy = acct_data.dummy.checked_add(1).unwrap();
        Ok(())
    }
}

#[derive(Accounts)]
pub struct ExecuteMultiSigContext<'info> {
    /// CHECK: expecting ExecuteMultiSigAccount but using UncheckedAccount
    pub account_a: UncheckedAccount<'info>,
    /// CHECK: expecting ExecuteMultiSigAccount but using UncheckedAccount
    pub account_b: UncheckedAccount<'info>,
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct ExecuteMultiSigAccount {
    pub dummy: u64,
}