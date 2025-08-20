use anchor_lang::prelude::*;
declare_id!("Case0481111111111111111111111111111111111111");

#[program]
pub mod case048 {
    use super::*;
    pub fn execute_sale_token(ctx: Context<SaleTokenContext>) -> Result<()> {
        // Use Case 48: ソーシャルトークン販売（SaleToken）
        // Vulnerable: using UncheckedAccount where SaleTokenAccount is expected
        msg!("Executing execute_sale_token for ソーシャルトークン販売（SaleToken）");
        // Example logic (dummy operation)
        let mut acct_data = SaleTokenAccount::try_from(&ctx.accounts.account_a.to_account_info())?;
        acct_data.dummy = acct_data.dummy.checked_add(1).unwrap();
        Ok(())
    }
}

#[derive(Accounts)]
pub struct SaleTokenContext<'info> {
    /// CHECK: expecting SaleTokenAccount but using UncheckedAccount
    pub account_a: UncheckedAccount<'info>,
    /// CHECK: expecting SaleTokenAccount but using UncheckedAccount
    pub account_b: UncheckedAccount<'info>,
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct SaleTokenAccount {
    pub dummy: u64,
}