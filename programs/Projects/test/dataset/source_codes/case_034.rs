use anchor_lang::prelude::*;
declare_id!("Case0341111111111111111111111111111111111111");

#[program]
pub mod case034 {
    use super::*;
    pub fn execute_options(ctx: Context<OptionsContext>) -> Result<()> {
        // Use Case 34: オプション取引（Options）コール権行使
        // Vulnerable: using UncheckedAccount where OptionsAccount is expected
        msg!("Executing execute_options for オプション取引（Options）コール権行使");
        // Example logic (dummy operation)
        let mut acct_data = OptionsAccount::try_from(&ctx.accounts.account_a.to_account_info())?;
        acct_data.dummy = acct_data.dummy.checked_add(1).unwrap();
        Ok(())
    }
}

#[derive(Accounts)]
pub struct OptionsContext<'info> {
    /// CHECK: expecting OptionsAccount but using UncheckedAccount
    pub account_a: UncheckedAccount<'info>,
    /// CHECK: expecting OptionsAccount but using UncheckedAccount
    pub account_b: UncheckedAccount<'info>,
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct OptionsAccount {
    pub dummy: u64,
}