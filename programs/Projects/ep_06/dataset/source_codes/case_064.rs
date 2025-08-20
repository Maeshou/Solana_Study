use anchor_lang::prelude::*;
declare_id!("REAL0641111111111111111111111111111111111111");

#[program]
pub mod case064 {
    use super::*;
    pub fn execute_realestateswap(ctx: Context<RealEstateSwapContext>) -> Result<()> {
        // Tokenization logic
        let mut token = TokenAccount::try_from(&ctx.accounts.account_a.to_account_info())?;
        token.issued = token.issued.checked_add(1).unwrap();
        Ok(())
    }
}

#[derive(Accounts)]
pub struct RealEstateSwapContext<'info> {
    /// CHECK: expecting RealEstateSwapAccount but using UncheckedAccount
    pub account_a: UncheckedAccount<'info>,
    /// CHECK: expecting RealEstateSwapAccount but using UncheckedAccount
    pub account_b: UncheckedAccount<'info>,
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct RealEstateSwapAccount {
    pub dummy: u64,
    pub counter: u64,
    pub version: u8,
}