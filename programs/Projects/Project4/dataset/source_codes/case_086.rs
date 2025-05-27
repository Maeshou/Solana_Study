use anchor_lang::prelude::*;
use borsh::{BorshDeserialize, BorshSerialize};

declare_id!("F00000000000000000000000000000056");

#[program]
pub mod case086_permissions {
    use super::*;

    pub fn initialize_permissions(ctx: Context<Initialize086>, param: u64) -> Result<()> {
        // Vulnerable initialization without init check
        let mut data = PermissionsData::try_from_slice(&ctx.accounts.data_account.data.borrow())?;
        data.value = param;  // Overwrites existing data
        data.serialize(&mut *ctx.accounts.data_account.data.borrow_mut())?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize086<'info> {
    #[account(mut)]
    pub data_account: AccountInfo<'info>,  // No init guard
    pub authority: Signer<'info>,
}

#[derive(BorshSerialize, BorshDeserialize)]
pub struct PermissionsData {
    pub value: u64,
}
