use anchor_lang::prelude::*;
declare_id!("Case0791111111111111111111111111111111111111");

#[program]
pub mod case079 {
    use super::*;
    pub fn execute_verify_twin(ctx: Context<VerifyTwinContext>) -> Result<()> {
        // Use Case 79: デジタルツイン検証（VerifyTwin）
        // Vulnerable: using UncheckedAccount where VerifyTwinAccount is expected
        msg!("Executing execute_verify_twin for デジタルツイン検証（VerifyTwin）");
        // Example logic (dummy operation)
        let mut acct_data = VerifyTwinAccount::try_from(&ctx.accounts.account_a.to_account_info())?;
        acct_data.dummy = acct_data.dummy.checked_add(1).unwrap();
        Ok(())
    }
}

#[derive(Accounts)]
pub struct VerifyTwinContext<'info> {
    /// CHECK: expecting VerifyTwinAccount but using UncheckedAccount
    pub account_a: UncheckedAccount<'info>,
    /// CHECK: expecting VerifyTwinAccount but using UncheckedAccount
    pub account_b: UncheckedAccount<'info>,
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct VerifyTwinAccount {
    pub dummy: u64,
}