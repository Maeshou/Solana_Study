use anchor_lang::prelude::*;
declare_id!("Case0701111111111111111111111111111111111111");

#[program]
pub mod case070 {
    use super::*;
    pub fn execute_dissolve_fund(ctx: Context<DissolveFundContext>) -> Result<()> {
        // Use Case 70: 投資ファンド解散（DissolveFund）
        // Vulnerable: using UncheckedAccount where DissolveFundAccount is expected
        msg!("Executing execute_dissolve_fund for 投資ファンド解散（DissolveFund）");
        // Example logic (dummy operation)
        let mut acct_data = DissolveFundAccount::try_from(&ctx.accounts.account_a.to_account_info())?;
        acct_data.dummy = acct_data.dummy.checked_add(1).unwrap();
        Ok(())
    }
}

#[derive(Accounts)]
pub struct DissolveFundContext<'info> {
    /// CHECK: expecting DissolveFundAccount but using UncheckedAccount
    pub account_a: UncheckedAccount<'info>,
    /// CHECK: expecting DissolveFundAccount but using UncheckedAccount
    pub account_b: UncheckedAccount<'info>,
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DissolveFundAccount {
    pub dummy: u64,
}