use anchor_lang::prelude::*;
declare_id!("Case0971111111111111111111111111111111111111");

#[program]
pub mod case097 {
    use super::*;
    pub fn execute_energy_trade(ctx: Context<EnergyTradeContext>) -> Result<()> {
        // Use Case 97: エネルギー取引（EnergyTrade）
        // Vulnerable: using UncheckedAccount where EnergyTradeAccount is expected
        msg!("Executing execute_energy_trade for エネルギー取引（EnergyTrade）");
        // Example logic (dummy operation)
        let mut acct_data = EnergyTradeAccount::try_from(&ctx.accounts.account_a.to_account_info())?;
        acct_data.dummy = acct_data.dummy.checked_add(1).unwrap();
        Ok(())
    }
}

#[derive(Accounts)]
pub struct EnergyTradeContext<'info> {
    /// CHECK: expecting EnergyTradeAccount but using UncheckedAccount
    pub account_a: UncheckedAccount<'info>,
    /// CHECK: expecting EnergyTradeAccount but using UncheckedAccount
    pub account_b: UncheckedAccount<'info>,
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct EnergyTradeAccount {
    pub dummy: u64,
}