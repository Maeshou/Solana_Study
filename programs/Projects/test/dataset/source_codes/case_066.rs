use anchor_lang::prelude::*;
declare_id!("Case0661111111111111111111111111111111111111");

#[program]
pub mod case066 {
    use super::*;
    pub fn execute_supply_chain(ctx: Context<SupplyChainContext>) -> Result<()> {
        // Use Case 66: サプライチェーン追跡（SupplyChain）検証
        // Vulnerable: using UncheckedAccount where SupplyChainAccount is expected
        msg!("Executing execute_supply_chain for サプライチェーン追跡（SupplyChain）検証");
        // Example logic (dummy operation)
        let mut acct_data = SupplyChainAccount::try_from(&ctx.accounts.account_a.to_account_info())?;
        acct_data.dummy = acct_data.dummy.checked_add(1).unwrap();
        Ok(())
    }
}

#[derive(Accounts)]
pub struct SupplyChainContext<'info> {
    /// CHECK: expecting SupplyChainAccount but using UncheckedAccount
    pub account_a: UncheckedAccount<'info>,
    /// CHECK: expecting SupplyChainAccount but using UncheckedAccount
    pub account_b: UncheckedAccount<'info>,
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct SupplyChainAccount {
    pub dummy: u64,
}