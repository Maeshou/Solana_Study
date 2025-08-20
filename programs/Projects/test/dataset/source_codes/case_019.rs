use anchor_lang::prelude::*;
declare_id!("Case0191111111111111111111111111111111111111");

#[program]
pub mod case019 {
    use super::*;
    pub fn execute_multi_sig(ctx: Context<MultiSigContext>) -> Result<()> {
        // Use Case 19: マルチシグ署名ワレット（MultiSig）作成
        // Vulnerable: using UncheckedAccount where MultiSigAccount is expected
        msg!("Executing execute_multi_sig for マルチシグ署名ワレット（MultiSig）作成");
        // Example logic (dummy operation)
        let mut acct_data = MultiSigAccount::try_from(&ctx.accounts.account_a.to_account_info())?;
        acct_data.dummy = acct_data.dummy.checked_add(1).unwrap();
        Ok(())
    }
}

#[derive(Accounts)]
pub struct MultiSigContext<'info> {
    /// CHECK: expecting MultiSigAccount but using UncheckedAccount
    pub account_a: UncheckedAccount<'info>,
    /// CHECK: expecting MultiSigAccount but using UncheckedAccount
    pub account_b: UncheckedAccount<'info>,
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct MultiSigAccount {
    pub dummy: u64,
}