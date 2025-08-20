use anchor_lang::prelude::*;
declare_id!("Case0521111111111111111111111111111111111111");

#[program]
pub mod case052 {
    use super::*;
    pub fn execute_burn_token(ctx: Context<BurnTokenContext>) -> Result<()> {
        // Use Case 52: トークンバーン（BurnToken）
        // Vulnerable: using UncheckedAccount where BurnTokenAccount is expected
        msg!("Executing execute_burn_token for トークンバーン（BurnToken）");
        // Example logic (dummy operation)
        let mut acct_data = BurnTokenAccount::try_from(&ctx.accounts.account_a.to_account_info())?;
        acct_data.dummy = acct_data.dummy.checked_add(1).unwrap();
        Ok(())
    }
}

#[derive(Accounts)]
pub struct BurnTokenContext<'info> {
    /// CHECK: expecting BurnTokenAccount but using UncheckedAccount
    pub account_a: UncheckedAccount<'info>,
    /// CHECK: expecting BurnTokenAccount but using UncheckedAccount
    pub account_b: UncheckedAccount<'info>,
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct BurnTokenAccount {
    pub dummy: u64,
}