use anchor_lang::prelude::*;
declare_id!("Case0391111111111111111111111111111111111111");

#[program]
pub mod case039 {
    use super::*;
    pub fn execute_set_guarantor(ctx: Context<SetGuarantorContext>) -> Result<()> {
        // Use Case 39: 保証人設定（SetGuarantor）
        // Vulnerable: using UncheckedAccount where SetGuarantorAccount is expected
        msg!("Executing execute_set_guarantor for 保証人設定（SetGuarantor）");
        // Example logic (dummy operation)
        let mut acct_data = SetGuarantorAccount::try_from(&ctx.accounts.account_a.to_account_info())?;
        acct_data.dummy = acct_data.dummy.checked_add(1).unwrap();
        Ok(())
    }
}

#[derive(Accounts)]
pub struct SetGuarantorContext<'info> {
    /// CHECK: expecting SetGuarantorAccount but using UncheckedAccount
    pub account_a: UncheckedAccount<'info>,
    /// CHECK: expecting SetGuarantorAccount but using UncheckedAccount
    pub account_b: UncheckedAccount<'info>,
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct SetGuarantorAccount {
    pub dummy: u64,
}