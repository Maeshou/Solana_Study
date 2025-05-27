use anchor_lang::prelude::*;
use borsh::{BorshDeserialize, BorshSerialize};

declare_id!("F00000000000000000000000000000028");

#[program]
pub mod case040_airdrop_config {
    use super::*;

    pub fn initialize_airdrop_config(ctx: Context<Initialize040>, param: u64) -> Result<()> {
        // Vulnerable initialization without init check
        let mut data = AirdropConfigData::try_from_slice(&ctx.accounts.data_account.data.borrow())?;
        data.value = param;  // Overwrites existing data
        data.serialize(&mut *ctx.accounts.data_account.data.borrow_mut())?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize040<'info> {
    #[account(mut)]
    pub data_account: AccountInfo<'info>,  // No init guard
    pub authority: Signer<'info>,
}

#[derive(BorshSerialize, BorshDeserialize)]
pub struct AirdropConfigData {
    pub value: u64,
}
