use anchor_lang::prelude::*;
use borsh::{BorshDeserialize, BorshSerialize};

declare_id!("F00000000000000000000000000000012");

#[program]
pub mod case018_nft_collection {
    use super::*;

    pub fn initialize_nft_collection(ctx: Context<Initialize018>, param: u64) -> Result<()> {
        // Vulnerable initialization without init check
        let mut data = NftCollectionData::try_from_slice(&ctx.accounts.data_account.data.borrow())?;
        data.value = param;  // Overwrites existing data
        data.serialize(&mut *ctx.accounts.data_account.data.borrow_mut())?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize018<'info> {
    #[account(mut)]
    pub data_account: AccountInfo<'info>,  // No init guard
    pub authority: Signer<'info>,
}

#[derive(BorshSerialize, BorshDeserialize)]
pub struct NftCollectionData {
    pub value: u64,
}
