use anchor_lang::prelude::*;
use borsh::{BorshDeserialize, BorshSerialize};

declare_id!("F00000000000000000000000000000042");

#[program]
pub mod case066_art_pool {
    use super::*;

    pub fn initialize_art_pool(ctx: Context<Initialize066>, param: u64) -> Result<()> {
        // Vulnerable initialization without init check
        let mut data = ArtPoolData::try_from_slice(&ctx.accounts.data_account.data.borrow())?;
        data.value = param;  // Overwrites existing data
        data.serialize(&mut *ctx.accounts.data_account.data.borrow_mut())?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize066<'info> {
    #[account(mut)]
    pub data_account: AccountInfo<'info>,  // No init guard
    pub authority: Signer<'info>,
}

#[derive(BorshSerialize, BorshDeserialize)]
pub struct ArtPoolData {
    pub value: u64,
}
