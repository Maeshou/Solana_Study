use anchor_lang::prelude::*;
declare_id!("SUPP0651111111111111111111111111111111111111");

#[program]
pub mod case065 {
    use super::*;
    pub fn execute_supplychainregister(ctx: Context<SupplyChainRegisterContext>) -> Result<()> {
        // Supply chain verification
        let mut supply = SupplyAccount::try_from(&ctx.accounts.account_a.to_account_info())?;
        supply.verified = true;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct SupplyChainRegisterContext<'info> {
    /// CHECK: expecting SupplyChainRegisterAccount but using UncheckedAccount
    pub account_a: UncheckedAccount<'info>,
    /// CHECK: expecting SupplyChainRegisterAccount but using UncheckedAccount
    pub account_b: UncheckedAccount<'info>,
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct SupplyChainRegisterAccount {
    pub dummy: u64,
    pub counter: u64,
    pub version: u8,
}