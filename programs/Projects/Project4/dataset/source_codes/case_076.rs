use anchor_lang::prelude::*;
use borsh::{BorshDeserialize, BorshSerialize};

declare_id!("F0000000000000000000000000000004c");

#[program]
pub mod case076_burn_authority {
    use super::*;

    pub fn initialize_burn_authority(ctx: Context<Initialize076>, param: u64) -> Result<()> {
        // Vulnerable initialization without init check
        let mut data = BurnAuthorityData::try_from_slice(&ctx.accounts.data_account.data.borrow())?;
        data.value = param;  // Overwrites existing data
        data.serialize(&mut *ctx.accounts.data_account.data.borrow_mut())?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize076<'info> {
    #[account(mut)]
    pub data_account: AccountInfo<'info>,  // No init guard
    pub authority: Signer<'info>,
}

#[derive(BorshSerialize, BorshDeserialize)]
pub struct BurnAuthorityData {
    pub value: u64,
}
