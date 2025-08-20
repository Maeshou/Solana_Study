use anchor_lang::prelude::*;
declare_id!("Case0491111111111111111111111111111111111111");

#[program]
pub mod case049 {
    use super::*;
    pub fn execute_buy_token(ctx: Context<BuyTokenContext>) -> Result<()> {
        // Use Case 49: ソーシャルトークン購入（BuyToken）
        // Vulnerable: using UncheckedAccount where BuyTokenAccount is expected
        msg!("Executing execute_buy_token for ソーシャルトークン購入（BuyToken）");
        // Example logic (dummy operation)
        let mut acct_data = BuyTokenAccount::try_from(&ctx.accounts.account_a.to_account_info())?;
        acct_data.dummy = acct_data.dummy.checked_add(1).unwrap();
        Ok(())
    }
}

#[derive(Accounts)]
pub struct BuyTokenContext<'info> {
    /// CHECK: expecting BuyTokenAccount but using UncheckedAccount
    pub account_a: UncheckedAccount<'info>,
    /// CHECK: expecting BuyTokenAccount but using UncheckedAccount
    pub account_b: UncheckedAccount<'info>,
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct BuyTokenAccount {
    pub dummy: u64,
}