use anchor_lang::prelude::*;
declare_id!("Case0941111111111111111111111111111111111111");

#[program]
pub mod case094 {
    use super::*;
    pub fn execute_regen_token(ctx: Context<RegenTokenContext>) -> Result<()> {
        // Use Case 94: 再生医療トークン（RegenToken）発行
        // Vulnerable: using UncheckedAccount where RegenTokenAccount is expected
        msg!("Executing execute_regen_token for 再生医療トークン（RegenToken）発行");
        // Example logic (dummy operation)
        let mut acct_data = RegenTokenAccount::try_from(&ctx.accounts.account_a.to_account_info())?;
        acct_data.dummy = acct_data.dummy.checked_add(1).unwrap();
        Ok(())
    }
}

#[derive(Accounts)]
pub struct RegenTokenContext<'info> {
    /// CHECK: expecting RegenTokenAccount but using UncheckedAccount
    pub account_a: UncheckedAccount<'info>,
    /// CHECK: expecting RegenTokenAccount but using UncheckedAccount
    pub account_b: UncheckedAccount<'info>,
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct RegenTokenAccount {
    pub dummy: u64,
}