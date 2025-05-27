use anchor_lang::prelude::*;
use borsh::{BorshDeserialize, BorshSerialize};

declare_id!("F00000000000000000000000000000001");

#[program]
pub mod case001_user_profile {
    use super::*;

    pub fn initialize_user_profile(ctx: Context<Initialize001>, param: u64) -> Result<()> {
        // Vulnerable initialization without init check
        let mut data = UserProfileData::try_from_slice(&ctx.accounts.data_account.data.borrow())?;
        data.value = param;  // Overwrites existing data
        data.serialize(&mut *ctx.accounts.data_account.data.borrow_mut())?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize001<'info> {
    #[account(mut)]
    pub data_account: AccountInfo<'info>,  // No init guard
    pub authority: Signer<'info>,
}

#[derive(BorshSerialize, BorshDeserialize)]
pub struct UserProfileData {
    pub value: u64,
}
