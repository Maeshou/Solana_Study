use anchor_lang::prelude::*;
use borsh::{BorshDeserialize, BorshSerialize};

declare_id!("F0000000000000000000000000000003a");

#[program]
pub mod case058_price_floor {
    use super::*;

    pub fn initialize_price_floor(ctx: Context<Initialize058>, param: u64) -> Result<()> {
        // Vulnerable initialization without init check
        let mut data = PriceFloorData::try_from_slice(&ctx.accounts.data_account.data.borrow())?;
        data.value = param;  // Overwrites existing data
        data.serialize(&mut *ctx.accounts.data_account.data.borrow_mut())?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize058<'info> {
    #[account(mut)]
    pub data_account: AccountInfo<'info>,  // No init guard
    pub authority: Signer<'info>,
}

#[derive(BorshSerialize, BorshDeserialize)]
pub struct PriceFloorData {
    pub value: u64,
}
