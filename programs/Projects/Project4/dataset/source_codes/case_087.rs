use anchor_lang::prelude::*;
use borsh::{BorshDeserialize, BorshSerialize};

declare_id!("F00000000000000000000000000000057");

#[program]
pub mod case087_identity_management {
    use super::*;

    pub fn initialize_identity_management(ctx: Context<Initialize087>, param: u64) -> Result<()> {
        // Vulnerable initialization without init check
        let mut data = IdentityManagementData::try_from_slice(&ctx.accounts.data_account.data.borrow())?;
        data.value = param;  // Overwrites existing data
        data.serialize(&mut *ctx.accounts.data_account.data.borrow_mut())?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize087<'info> {
    #[account(mut)]
    pub data_account: AccountInfo<'info>,  // No init guard
    pub authority: Signer<'info>,
}

#[derive(BorshSerialize, BorshDeserialize)]
pub struct IdentityManagementData {
    pub value: u64,
}
