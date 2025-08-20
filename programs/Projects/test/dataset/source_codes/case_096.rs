use anchor_lang::prelude::*;
declare_id!("Case0961111111111111111111111111111111111111");

#[program]
pub mod case096 {
    use super::*;
    pub fn execute_energy_grid(ctx: Context<EnergyGridContext>) -> Result<()> {
        // Use Case 96: エネルギー分散ネットワーク（EnergyGrid）グリッド登録
        // Vulnerable: using UncheckedAccount where EnergyGridAccount is expected
        msg!("Executing execute_energy_grid for エネルギー分散ネットワーク（EnergyGrid）グリッド登録");
        // Example logic (dummy operation)
        let mut acct_data = EnergyGridAccount::try_from(&ctx.accounts.account_a.to_account_info())?;
        acct_data.dummy = acct_data.dummy.checked_add(1).unwrap();
        Ok(())
    }
}

#[derive(Accounts)]
pub struct EnergyGridContext<'info> {
    /// CHECK: expecting EnergyGridAccount but using UncheckedAccount
    pub account_a: UncheckedAccount<'info>,
    /// CHECK: expecting EnergyGridAccount but using UncheckedAccount
    pub account_b: UncheckedAccount<'info>,
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct EnergyGridAccount {
    pub dummy: u64,
}