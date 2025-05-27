use anchor_lang::prelude::*;
use borsh::{BorshDeserialize, BorshSerialize};

declare_id!("F00000000000000000000000000000020");

#[program]
pub mod case032_points_system {
    use super::*;

    pub fn initialize_points_system(ctx: Context<Initialize032>, param: u64) -> Result<()> {
        // Vulnerable initialization without init check
        let mut data = PointsSystemData::try_from_slice(&ctx.accounts.data_account.data.borrow())?;
        data.value = param;  // Overwrites existing data
        data.serialize(&mut *ctx.accounts.data_account.data.borrow_mut())?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize032<'info> {
    #[account(mut)]
    pub data_account: AccountInfo<'info>,  // No init guard
    pub authority: Signer<'info>,
}

#[derive(BorshSerialize, BorshDeserialize)]
pub struct PointsSystemData {
    pub value: u64,
}
