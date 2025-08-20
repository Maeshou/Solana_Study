use anchor_lang::prelude::*;
declare_id!("Case0021111111111111111111111111111111111111");

#[program]
pub mod case002 {
    use super::*;
    pub fn execute_claim(ctx: Context<ClaimContext>) -> Result<()> {
        // Use Case 2: トークン受け取り（Claim）
        // Vulnerable: using UncheckedAccount where ClaimAccount is expected
        msg!("Executing execute_claim for トークン受け取り（Claim）");
        // Example logic (dummy operation)
        let mut acct_data = ClaimAccount::try_from(&ctx.accounts.account_a.to_account_info())?;
        acct_data.dummy = acct_data.dummy.checked_add(1).unwrap();
        Ok(())
    }
}

#[derive(Accounts)]
pub struct ClaimContext<'info> {
    /// CHECK: expecting ClaimAccount but using UncheckedAccount
    pub account_a: UncheckedAccount<'info>,
    /// CHECK: expecting ClaimAccount but using UncheckedAccount
    pub account_b: UncheckedAccount<'info>,
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct ClaimAccount {
    pub dummy: u64,
}