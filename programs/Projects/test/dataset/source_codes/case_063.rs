use anchor_lang::prelude::*;
declare_id!("Case0631111111111111111111111111111111111111");

#[program]
pub mod case063 {
    use super::*;
    pub fn execute_real_estate_tokenize(ctx: Context<RealEstateTokenizeContext>) -> Result<()> {
        // Use Case 63: 不動産トークン化（RealEstateTokenize）
        // Vulnerable: using UncheckedAccount where RealEstateTokenizeAccount is expected
        msg!("Executing execute_real_estate_tokenize for 不動産トークン化（RealEstateTokenize）");
        // Example logic (dummy operation)
        let mut acct_data = RealEstateTokenizeAccount::try_from(&ctx.accounts.account_a.to_account_info())?;
        acct_data.dummy = acct_data.dummy.checked_add(1).unwrap();
        Ok(())
    }
}

#[derive(Accounts)]
pub struct RealEstateTokenizeContext<'info> {
    /// CHECK: expecting RealEstateTokenizeAccount but using UncheckedAccount
    pub account_a: UncheckedAccount<'info>,
    /// CHECK: expecting RealEstateTokenizeAccount but using UncheckedAccount
    pub account_b: UncheckedAccount<'info>,
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct RealEstateTokenizeAccount {
    pub dummy: u64,
}