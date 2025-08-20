use anchor_lang::prelude::*;
declare_id!("Case0821111111111111111111111111111111111111");

#[program]
pub mod case082 {
    use super::*;
    pub fn execute_carbon_credit(ctx: Context<CarbonCreditContext>) -> Result<()> {
        // Use Case 82: 気候クレジット取引（CarbonCredit）
        // Vulnerable: using UncheckedAccount where CarbonCreditAccount is expected
        msg!("Executing execute_carbon_credit for 気候クレジット取引（CarbonCredit）");
        // Example logic (dummy operation)
        let mut acct_data = CarbonCreditAccount::try_from(&ctx.accounts.account_a.to_account_info())?;
        acct_data.dummy = acct_data.dummy.checked_add(1).unwrap();
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CarbonCreditContext<'info> {
    /// CHECK: expecting CarbonCreditAccount but using UncheckedAccount
    pub account_a: UncheckedAccount<'info>,
    /// CHECK: expecting CarbonCreditAccount but using UncheckedAccount
    pub account_b: UncheckedAccount<'info>,
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct CarbonCreditAccount {
    pub dummy: u64,
}