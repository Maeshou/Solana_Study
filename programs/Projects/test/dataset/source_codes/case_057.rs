use anchor_lang::prelude::*;
declare_id!("Case0571111111111111111111111111111111111111");

#[program]
pub mod case057 {
    use super::*;
    pub fn execute_register_referral(ctx: Context<RegisterReferralContext>) -> Result<()> {
        // Use Case 57: リファラル報酬登録（RegisterReferral）
        // Vulnerable: using UncheckedAccount where RegisterReferralAccount is expected
        msg!("Executing execute_register_referral for リファラル報酬登録（RegisterReferral）");
        // Example logic (dummy operation)
        let mut acct_data = RegisterReferralAccount::try_from(&ctx.accounts.account_a.to_account_info())?;
        acct_data.dummy = acct_data.dummy.checked_add(1).unwrap();
        Ok(())
    }
}

#[derive(Accounts)]
pub struct RegisterReferralContext<'info> {
    /// CHECK: expecting RegisterReferralAccount but using UncheckedAccount
    pub account_a: UncheckedAccount<'info>,
    /// CHECK: expecting RegisterReferralAccount but using UncheckedAccount
    pub account_b: UncheckedAccount<'info>,
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct RegisterReferralAccount {
    pub dummy: u64,
}