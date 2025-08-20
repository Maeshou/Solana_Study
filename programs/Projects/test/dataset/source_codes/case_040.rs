use anchor_lang::prelude::*;
declare_id!("Case0401111111111111111111111111111111111111");

#[program]
pub mod case040 {
    use super::*;
    pub fn execute_approve_guarantor(ctx: Context<ApproveGuarantorContext>) -> Result<()> {
        // Use Case 40: 保証人承認（ApproveGuarantor）
        // Vulnerable: using UncheckedAccount where ApproveGuarantorAccount is expected
        msg!("Executing execute_approve_guarantor for 保証人承認（ApproveGuarantor）");
        // Example logic (dummy operation)
        let mut acct_data = ApproveGuarantorAccount::try_from(&ctx.accounts.account_a.to_account_info())?;
        acct_data.dummy = acct_data.dummy.checked_add(1).unwrap();
        Ok(())
    }
}

#[derive(Accounts)]
pub struct ApproveGuarantorContext<'info> {
    /// CHECK: expecting ApproveGuarantorAccount but using UncheckedAccount
    pub account_a: UncheckedAccount<'info>,
    /// CHECK: expecting ApproveGuarantorAccount but using UncheckedAccount
    pub account_b: UncheckedAccount<'info>,
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct ApproveGuarantorAccount {
    pub dummy: u64,
}