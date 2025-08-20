use anchor_lang::prelude::*;
declare_id!("TOKE0631111111111111111111111111111111111111");

#[program]
pub mod case063 {
    use super::*;
    pub fn execute_tokenizerealestate(ctx: Context<TokenizeRealEstateContext>) -> Result<()> {
        // Tokenization logic
        let mut token = TokenAccount::try_from(&ctx.accounts.account_a.to_account_info())?;
        token.issued = token.issued.checked_add(1).unwrap();
        Ok(())
    }
}

#[derive(Accounts)]
pub struct TokenizeRealEstateContext<'info> {
    /// CHECK: expecting TokenizeRealEstateAccount but using UncheckedAccount
    pub account_a: UncheckedAccount<'info>,
    /// CHECK: expecting TokenizeRealEstateAccount but using UncheckedAccount
    pub account_b: UncheckedAccount<'info>,
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct TokenizeRealEstateAccount {
    pub dummy: u64,
    pub counter: u64,
    pub version: u8,
}