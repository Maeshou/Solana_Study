use anchor_lang::prelude::*;
use borsh::{BorshDeserialize, BorshSerialize};

declare_id!("F0000000000000000000000000000002b");

#[program]
pub mod case043_collateral_pool {
    use super::*;

    pub fn initialize_collateral_pool(ctx: Context<Initialize043>, param: u64) -> Result<()> {
        // Vulnerable initialization without init check
        let mut data = CollateralPoolData::try_from_slice(&ctx.accounts.data_account.data.borrow())?;
        data.value = param;  // Overwrites existing data
        data.serialize(&mut *ctx.accounts.data_account.data.borrow_mut())?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize043<'info> {
    #[account(mut)]
    pub data_account: AccountInfo<'info>,  // No init guard
    pub authority: Signer<'info>,
}

#[derive(BorshSerialize, BorshDeserialize)]
pub struct CollateralPoolData {
    pub value: u64,
}
