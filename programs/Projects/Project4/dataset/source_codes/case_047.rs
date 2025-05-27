use anchor_lang::prelude::*;
use borsh::{BorshDeserialize, BorshSerialize};

declare_id!("F0000000000000000000000000000002f");

#[program]
pub mod case047_security_policy {
    use super::*;

    pub fn initialize_security_policy(ctx: Context<Initialize047>, param: u64) -> Result<()> {
        // Vulnerable initialization without init check
        let mut data = SecurityPolicyData::try_from_slice(&ctx.accounts.data_account.data.borrow())?;
        data.value = param;  // Overwrites existing data
        data.serialize(&mut *ctx.accounts.data_account.data.borrow_mut())?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize047<'info> {
    #[account(mut)]
    pub data_account: AccountInfo<'info>,  // No init guard
    pub authority: Signer<'info>,
}

#[derive(BorshSerialize, BorshDeserialize)]
pub struct SecurityPolicyData {
    pub value: u64,
}
