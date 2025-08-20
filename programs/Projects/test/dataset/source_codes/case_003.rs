use anchor_lang::prelude::*;
declare_id!("Case0031111111111111111111111111111111111111");

#[program]
pub mod case003 {
    use super::*;
    pub fn execute_一時保管(ctx: Context<一時保管Context>) -> Result<()> {
        // Use Case 3: エスクロー（一時保管）
        // Vulnerable: using UncheckedAccount where 一時保管Account is expected
        msg!("Executing execute_一時保管 for エスクロー（一時保管）");
        // Example logic (dummy operation)
        let mut acct_data = 一時保管Account::try_from(&ctx.accounts.account_a.to_account_info())?;
        acct_data.dummy = acct_data.dummy.checked_add(1).unwrap();
        Ok(())
    }
}

#[derive(Accounts)]
pub struct 一時保管Context<'info> {
    /// CHECK: expecting 一時保管Account but using UncheckedAccount
    pub account_a: UncheckedAccount<'info>,
    /// CHECK: expecting 一時保管Account but using UncheckedAccount
    pub account_b: UncheckedAccount<'info>,
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct 一時保管Account {
    pub dummy: u64,
}