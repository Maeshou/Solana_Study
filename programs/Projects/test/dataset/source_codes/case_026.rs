use anchor_lang::prelude::*;
declare_id!("Case0261111111111111111111111111111111111111");

#[program]
pub mod case026 {
    use super::*;
    pub fn execute_approve_claim(ctx: Context<ApproveClaimContext>) -> Result<()> {
        // Use Case 26: 保険承認（ApproveClaim）
        // Vulnerable: using UncheckedAccount where ApproveClaimAccount is expected
        msg!("Executing execute_approve_claim for 保険承認（ApproveClaim）");
        // Example logic (dummy operation)
        let mut acct_data = ApproveClaimAccount::try_from(&ctx.accounts.account_a.to_account_info())?;
        acct_data.dummy = acct_data.dummy.checked_add(1).unwrap();
        Ok(())
    }
}

#[derive(Accounts)]
pub struct ApproveClaimContext<'info> {
    /// CHECK: expecting ApproveClaimAccount but using UncheckedAccount
    pub account_a: UncheckedAccount<'info>,
    /// CHECK: expecting ApproveClaimAccount but using UncheckedAccount
    pub account_b: UncheckedAccount<'info>,
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct ApproveClaimAccount {
    pub dummy: u64,
}