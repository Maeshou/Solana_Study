use anchor_lang::prelude::*;
declare_id!("Case0251111111111111111111111111111111111111");

#[program]
pub mod case025 {
    use super::*;
    pub fn execute_submit_claim(ctx: Context<SubmitClaimContext>) -> Result<()> {
        // Use Case 25: 保険請求（SubmitClaim）
        // Vulnerable: using UncheckedAccount where SubmitClaimAccount is expected
        msg!("Executing execute_submit_claim for 保険請求（SubmitClaim）");
        // Example logic (dummy operation)
        let mut acct_data = SubmitClaimAccount::try_from(&ctx.accounts.account_a.to_account_info())?;
        acct_data.dummy = acct_data.dummy.checked_add(1).unwrap();
        Ok(())
    }
}

#[derive(Accounts)]
pub struct SubmitClaimContext<'info> {
    /// CHECK: expecting SubmitClaimAccount but using UncheckedAccount
    pub account_a: UncheckedAccount<'info>,
    /// CHECK: expecting SubmitClaimAccount but using UncheckedAccount
    pub account_b: UncheckedAccount<'info>,
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct SubmitClaimAccount {
    pub dummy: u64,
}