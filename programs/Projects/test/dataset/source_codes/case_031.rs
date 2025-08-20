use anchor_lang::prelude::*;
declare_id!("Case0311111111111111111111111111111111111111");

#[program]
pub mod case031 {
    use super::*;
    pub fn execute_verify_did(ctx: Context<VerifyDIDContext>) -> Result<()> {
        // Use Case 31: DID 検証（VerifyDID）
        // Vulnerable: using UncheckedAccount where VerifyDIDAccount is expected
        msg!("Executing execute_verify_did for DID 検証（VerifyDID）");
        // Example logic (dummy operation)
        let mut acct_data = VerifyDIDAccount::try_from(&ctx.accounts.account_a.to_account_info())?;
        acct_data.dummy = acct_data.dummy.checked_add(1).unwrap();
        Ok(())
    }
}

#[derive(Accounts)]
pub struct VerifyDIDContext<'info> {
    /// CHECK: expecting VerifyDIDAccount but using UncheckedAccount
    pub account_a: UncheckedAccount<'info>,
    /// CHECK: expecting VerifyDIDAccount but using UncheckedAccount
    pub account_b: UncheckedAccount<'info>,
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct VerifyDIDAccount {
    pub dummy: u64,
}