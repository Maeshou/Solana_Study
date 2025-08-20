use anchor_lang::prelude::*;
declare_id!("Case0611111111111111111111111111111111111111");

#[program]
pub mod case061 {
    use super::*;
    pub fn execute_rwa_collateralize(ctx: Context<RWACollateralizeContext>) -> Result<()> {
        // Use Case 61: リアルワールドアセット担保（RWA Collateralize）
        // Vulnerable: using UncheckedAccount where RWACollateralizeAccount is expected
        msg!("Executing execute_rwa_collateralize for リアルワールドアセット担保（RWA Collateralize）");
        // Example logic (dummy operation)
        let mut acct_data = RWACollateralizeAccount::try_from(&ctx.accounts.account_a.to_account_info())?;
        acct_data.dummy = acct_data.dummy.checked_add(1).unwrap();
        Ok(())
    }
}

#[derive(Accounts)]
pub struct RWACollateralizeContext<'info> {
    /// CHECK: expecting RWACollateralizeAccount but using UncheckedAccount
    pub account_a: UncheckedAccount<'info>,
    /// CHECK: expecting RWACollateralizeAccount but using UncheckedAccount
    pub account_b: UncheckedAccount<'info>,
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct RWACollateralizeAccount {
    pub dummy: u64,
}