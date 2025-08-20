use anchor_lang::prelude::*;
declare_id!("Case0981111111111111111111111111111111111111");

#[program]
pub mod case098 {
    use super::*;
    pub fn execute_consume_energy_credit(ctx: Context<ConsumeEnergyCreditContext>) -> Result<()> {
        // Use Case 98: エネルギークレジット消費（ConsumeEnergyCredit）
        // Vulnerable: using UncheckedAccount where ConsumeEnergyCreditAccount is expected
        msg!("Executing execute_consume_energy_credit for エネルギークレジット消費（ConsumeEnergyCredit）");
        // Example logic (dummy operation)
        let mut acct_data = ConsumeEnergyCreditAccount::try_from(&ctx.accounts.account_a.to_account_info())?;
        acct_data.dummy = acct_data.dummy.checked_add(1).unwrap();
        Ok(())
    }
}

#[derive(Accounts)]
pub struct ConsumeEnergyCreditContext<'info> {
    /// CHECK: expecting ConsumeEnergyCreditAccount but using UncheckedAccount
    pub account_a: UncheckedAccount<'info>,
    /// CHECK: expecting ConsumeEnergyCreditAccount but using UncheckedAccount
    pub account_b: UncheckedAccount<'info>,
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct ConsumeEnergyCreditAccount {
    pub dummy: u64,
}