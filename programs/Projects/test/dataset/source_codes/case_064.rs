use anchor_lang::prelude::*;
declare_id!("Case0641111111111111111111111111111111111111");

#[program]
pub mod case064 {
    use super::*;
    pub fn execute_real_estate_swap(ctx: Context<RealEstateSwapContext>) -> Result<()> {
        // Use Case 64: 不動産トークン売買（RealEstateSwap）
        // Vulnerable: using UncheckedAccount where RealEstateSwapAccount is expected
        msg!("Executing execute_real_estate_swap for 不動産トークン売買（RealEstateSwap）");
        // Example logic (dummy operation)
        let mut acct_data = RealEstateSwapAccount::try_from(&ctx.accounts.account_a.to_account_info())?;
        acct_data.dummy = acct_data.dummy.checked_add(1).unwrap();
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
}