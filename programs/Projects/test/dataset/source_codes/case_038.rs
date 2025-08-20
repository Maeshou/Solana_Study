use anchor_lang::prelude::*;
declare_id!("Case0381111111111111111111111111111111111111");

#[program]
pub mod case038 {
    use super::*;
    pub fn execute_tokenize_debt(ctx: Context<TokenizeDebtContext>) -> Result<()> {
        // Use Case 38: 債権トークン化（TokenizeDebt）
        // Vulnerable: using UncheckedAccount where TokenizeDebtAccount is expected
        msg!("Executing execute_tokenize_debt for 債権トークン化（TokenizeDebt）");
        // Example logic (dummy operation)
        let mut acct_data = TokenizeDebtAccount::try_from(&ctx.accounts.account_a.to_account_info())?;
        acct_data.dummy = acct_data.dummy.checked_add(1).unwrap();
        Ok(())
    }
}

#[derive(Accounts)]
pub struct TokenizeDebtContext<'info> {
    /// CHECK: expecting TokenizeDebtAccount but using UncheckedAccount
    pub account_a: UncheckedAccount<'info>,
    /// CHECK: expecting TokenizeDebtAccount but using UncheckedAccount
    pub account_b: UncheckedAccount<'info>,
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct TokenizeDebtAccount {
    pub dummy: u64,
}