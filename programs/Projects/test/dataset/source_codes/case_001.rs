use anchor_lang::prelude::*;
declare_id!("Case0011111111111111111111111111111111111111");

#[program]
pub mod case001 {
    use super::*;
    pub fn execute_transfer(ctx: Context<TransferContext>) -> Result<()> {
        // Use Case 1: 単純なトークン送金（Transfer）
        // Vulnerable: using UncheckedAccount where TransferAccount is expected
        msg!("Executing execute_transfer for 単純なトークン送金（Transfer）");
        // Example logic (dummy operation)
        let mut acct_data = TransferAccount::try_from(&ctx.accounts.account_a.to_account_info())?;
        acct_data.dummy = acct_data.dummy.checked_add(1).unwrap();
        Ok(())
    }
}

#[derive(Accounts)]
pub struct TransferContext<'info> {
    /// CHECK: expecting TransferAccount but using UncheckedAccount
    pub account_a: UncheckedAccount<'info>,
    /// CHECK: expecting TransferAccount but using UncheckedAccount
    pub account_b: UncheckedAccount<'info>,
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct TransferAccount {
    pub dummy: u64,
}