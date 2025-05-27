use anchor_lang::prelude::*;
use borsh::{BorshDeserialize, BorshSerialize};

declare_id!("F00000000000000000000000000000041");

#[program]
pub mod case065_nft_auction {
    use super::*;

    pub fn initialize_nft_auction(ctx: Context<Initialize065>, param: u64) -> Result<()> {
        // Vulnerable initialization without init check
        let mut data = NftAuctionData::try_from_slice(&ctx.accounts.data_account.data.borrow())?;
        data.value = param;  // Overwrites existing data
        data.serialize(&mut *ctx.accounts.data_account.data.borrow_mut())?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize065<'info> {
    #[account(mut)]
    pub data_account: AccountInfo<'info>,  // No init guard
    pub authority: Signer<'info>,
}

#[derive(BorshSerialize, BorshDeserialize)]
pub struct NftAuctionData {
    pub value: u64,
}
