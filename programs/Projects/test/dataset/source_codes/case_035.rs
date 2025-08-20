use anchor_lang::prelude::*;
declare_id!("Case0351111111111111111111111111111111111111");

#[program]
pub mod case035 {
    use super::*;
    pub fn execute_case35(ctx: Context<Case35Context>) -> Result<()> {
        // Use Case 35: オプション取引プット権行使
        // Vulnerable: using UncheckedAccount where Case35Account is expected
        msg!("Executing execute_case35 for オプション取引プット権行使");
        // Example logic (dummy operation)
        let mut acct_data = Case35Account::try_from(&ctx.accounts.account_a.to_account_info())?;
        acct_data.dummy = acct_data.dummy.checked_add(1).unwrap();
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Case35Context<'info> {
    /// CHECK: expecting Case35Account but using UncheckedAccount
    pub account_a: UncheckedAccount<'info>,
    /// CHECK: expecting Case35Account but using UncheckedAccount
    pub account_b: UncheckedAccount<'info>,
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Case35Account {
    pub dummy: u64,
}