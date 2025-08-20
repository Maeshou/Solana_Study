use anchor_lang::prelude::*;
declare_id!("Case0621111111111111111111111111111111111111");

#[program]
pub mod case062 {
    use super::*;
    pub fn execute_rwa_evaluate(ctx: Context<RWAEvaluateContext>) -> Result<()> {
        // Use Case 62: リアルワールドアセット評価（RWA Evaluate）
        // Vulnerable: using UncheckedAccount where RWAEvaluateAccount is expected
        msg!("Executing execute_rwa_evaluate for リアルワールドアセット評価（RWA Evaluate）");
        // Example logic (dummy operation)
        let mut acct_data = RWAEvaluateAccount::try_from(&ctx.accounts.account_a.to_account_info())?;
        acct_data.dummy = acct_data.dummy.checked_add(1).unwrap();
        Ok(())
    }
}

#[derive(Accounts)]
pub struct RWAEvaluateContext<'info> {
    /// CHECK: expecting RWAEvaluateAccount but using UncheckedAccount
    pub account_a: UncheckedAccount<'info>,
    /// CHECK: expecting RWAEvaluateAccount but using UncheckedAccount
    pub account_b: UncheckedAccount<'info>,
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct RWAEvaluateAccount {
    pub dummy: u64,
}