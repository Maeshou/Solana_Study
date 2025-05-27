use anchor_lang::prelude::*;
use borsh::{BorshDeserialize, BorshSerialize};

declare_id!("F00000000000000000000000000000035");

#[program]
pub mod case053_bid_order {
    use super::*;

    pub fn initialize_bid_order(ctx: Context<Initialize053>, param: u64) -> Result<()> {
        // Vulnerable initialization without init check
        let mut data = BidOrderData::try_from_slice(&ctx.accounts.data_account.data.borrow())?;
        data.value = param;  // Overwrites existing data
        data.serialize(&mut *ctx.accounts.data_account.data.borrow_mut())?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize053<'info> {
    #[account(mut)]
    pub data_account: AccountInfo<'info>,  // No init guard
    pub authority: Signer<'info>,
}

#[derive(BorshSerialize, BorshDeserialize)]
pub struct BidOrderData {
    pub value: u64,
}
