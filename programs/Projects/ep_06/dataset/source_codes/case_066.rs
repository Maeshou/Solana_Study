use anchor_lang::prelude::*;
declare_id!("SUPP0661111111111111111111111111111111111111");

#[program]
pub mod case066 {
    use super::*;
    pub fn execute_supplychainverify(ctx: Context<SupplyChainVerifyContext>) -> Result<()> {
        // Supply chain verification
        let mut supply = SupplyAccount::try_from(&ctx.accounts.account_a.to_account_info())?;
        supply.verified = true;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct SupplyChainVerifyContext<'info> {
    /// CHECK: expecting SupplyChainVerifyAccount but using UncheckedAccount
    pub account_a: UncheckedAccount<'info>,
    /// CHECK: expecting SupplyChainVerifyAccount but using UncheckedAccount
    pub account_b: UncheckedAccount<'info>,
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct SupplyChainVerifyAccount {
    pub dummy: u64,
    pub counter: u64,
    pub version: u8,
}