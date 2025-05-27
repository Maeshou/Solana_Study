use anchor_lang::prelude::*;
use borsh::{BorshDeserialize, BorshSerialize};

declare_id!("F0000000000000000000000000000004a");

#[program]
pub mod case074_pool_distribution {
    use super::*;

    pub fn initialize_pool_distribution(ctx: Context<Initialize074>, param: u64) -> Result<()> {
        // Vulnerable initialization without init check
        let mut data = PoolDistributionData::try_from_slice(&ctx.accounts.data_account.data.borrow())?;
        data.value = param;  // Overwrites existing data
        data.serialize(&mut *ctx.accounts.data_account.data.borrow_mut())?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize074<'info> {
    #[account(mut)]
    pub data_account: AccountInfo<'info>,  // No init guard
    pub authority: Signer<'info>,
}

#[derive(BorshSerialize, BorshDeserialize)]
pub struct PoolDistributionData {
    pub value: u64,
}
