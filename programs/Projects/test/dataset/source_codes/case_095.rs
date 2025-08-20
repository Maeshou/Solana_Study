use anchor_lang::prelude::*;
declare_id!("Case0951111111111111111111111111111111111111");

#[program]
pub mod case095 {
    use super::*;
    pub fn execute_use_regen_token(ctx: Context<UseRegenTokenContext>) -> Result<()> {
        // Use Case 95: 再生医療トークン使用（UseRegenToken）
        // Vulnerable: using UncheckedAccount where UseRegenTokenAccount is expected
        msg!("Executing execute_use_regen_token for 再生医療トークン使用（UseRegenToken）");
        // Example logic (dummy operation)
        let mut acct_data = UseRegenTokenAccount::try_from(&ctx.accounts.account_a.to_account_info())?;
        acct_data.dummy = acct_data.dummy.checked_add(1).unwrap();
        Ok(())
    }
}

#[derive(Accounts)]
pub struct UseRegenTokenContext<'info> {
    /// CHECK: expecting UseRegenTokenAccount but using UncheckedAccount
    pub account_a: UncheckedAccount<'info>,
    /// CHECK: expecting UseRegenTokenAccount but using UncheckedAccount
    pub account_b: UncheckedAccount<'info>,
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct UseRegenTokenAccount {
    pub dummy: u64,
}