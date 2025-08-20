use anchor_lang::prelude::*;
declare_id!("Case0881111111111111111111111111111111111111");

#[program]
pub mod case088 {
    use super::*;
    pub fn execute_case88(ctx: Context<Case88Context>) -> Result<()> {
        // Use Case 88: フラッシュローン返済
        // Vulnerable: using UncheckedAccount where Case88Account is expected
        msg!("Executing execute_case88 for フラッシュローン返済");
        // Example logic (dummy operation)
        let mut acct_data = Case88Account::try_from(&ctx.accounts.account_a.to_account_info())?;
        acct_data.dummy = acct_data.dummy.checked_add(1).unwrap();
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Case88Context<'info> {
    /// CHECK: expecting Case88Account but using UncheckedAccount
    pub account_a: UncheckedAccount<'info>,
    /// CHECK: expecting Case88Account but using UncheckedAccount
    pub account_b: UncheckedAccount<'info>,
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Case88Account {
    pub dummy: u64,
}