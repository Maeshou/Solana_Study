use anchor_lang::prelude::*;
declare_id!("Case0301111111111111111111111111111111111111");

#[program]
pub mod case030 {
    use super::*;
    pub fn execute_decentralized_id(ctx: Context<DecentralizedIDContext>) -> Result<()> {
        // Use Case 30: DID（Decentralized ID）登録
        // Vulnerable: using UncheckedAccount where DecentralizedIDAccount is expected
        msg!("Executing execute_decentralized_id for DID（Decentralized ID）登録");
        // Example logic (dummy operation)
        let mut acct_data = DecentralizedIDAccount::try_from(&ctx.accounts.account_a.to_account_info())?;
        acct_data.dummy = acct_data.dummy.checked_add(1).unwrap();
        Ok(())
    }
}

#[derive(Accounts)]
pub struct DecentralizedIDContext<'info> {
    /// CHECK: expecting DecentralizedIDAccount but using UncheckedAccount
    pub account_a: UncheckedAccount<'info>,
    /// CHECK: expecting DecentralizedIDAccount but using UncheckedAccount
    pub account_b: UncheckedAccount<'info>,
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DecentralizedIDAccount {
    pub dummy: u64,
}