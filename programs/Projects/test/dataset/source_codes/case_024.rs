use anchor_lang::prelude::*;
declare_id!("Case0241111111111111111111111111111111111111");

#[program]
pub mod case024 {
    use super::*;
    pub fn execute_place_bet(ctx: Context<PlaceBetContext>) -> Result<()> {
        // Use Case 24: ベット（PlaceBet）
        // Vulnerable: using UncheckedAccount where PlaceBetAccount is expected
        msg!("Executing execute_place_bet for ベット（PlaceBet）");
        // Example logic (dummy operation)
        let mut acct_data = PlaceBetAccount::try_from(&ctx.accounts.account_a.to_account_info())?;
        acct_data.dummy = acct_data.dummy.checked_add(1).unwrap();
        Ok(())
    }
}

#[derive(Accounts)]
pub struct PlaceBetContext<'info> {
    /// CHECK: expecting PlaceBetAccount but using UncheckedAccount
    pub account_a: UncheckedAccount<'info>,
    /// CHECK: expecting PlaceBetAccount but using UncheckedAccount
    pub account_b: UncheckedAccount<'info>,
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct PlaceBetAccount {
    pub dummy: u64,
}