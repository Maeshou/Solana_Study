use anchor_lang::prelude::*;
declare_id!("Case0741111111111111111111111111111111111111");

#[program]
pub mod case074 {
    use super::*;
    pub fn execute_sell_content(ctx: Context<SellContentContext>) -> Result<()> {
        // Use Case 74: コンテンツ販売（SellContent）
        // Vulnerable: using UncheckedAccount where SellContentAccount is expected
        msg!("Executing execute_sell_content for コンテンツ販売（SellContent）");
        // Example logic (dummy operation)
        let mut acct_data = SellContentAccount::try_from(&ctx.accounts.account_a.to_account_info())?;
        acct_data.dummy = acct_data.dummy.checked_add(1).unwrap();
        Ok(())
    }
}

#[derive(Accounts)]
pub struct SellContentContext<'info> {
    /// CHECK: expecting SellContentAccount but using UncheckedAccount
    pub account_a: UncheckedAccount<'info>,
    /// CHECK: expecting SellContentAccount but using UncheckedAccount
    pub account_b: UncheckedAccount<'info>,
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct SellContentAccount {
    pub dummy: u64,
}