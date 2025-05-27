use anchor_lang::prelude::*;
use borsh::{BorshDeserialize, BorshSerialize};

declare_id!("F00000000000000000000000000000061");

#[program]
pub mod case097_state_sync {
    use super::*;

    pub fn initialize_state_sync(ctx: Context<Initialize097>, param: u64) -> Result<()> {
        // Vulnerable initialization without init check
        let mut data = StateSyncData::try_from_slice(&ctx.accounts.data_account.data.borrow())?;
        data.value = param;  // Overwrites existing data
        data.serialize(&mut *ctx.accounts.data_account.data.borrow_mut())?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize097<'info> {
    #[account(mut)]
    pub data_account: AccountInfo<'info>,  // No init guard
    pub authority: Signer<'info>,
}

#[derive(BorshSerialize, BorshDeserialize)]
pub struct StateSyncData {
    pub value: u64,
}
