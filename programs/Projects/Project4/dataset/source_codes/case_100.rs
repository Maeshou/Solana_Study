use anchor_lang::prelude::*;
use borsh::{BorshDeserialize, BorshSerialize};

declare_id!("F00000000000000000000000000000064");

#[program]
pub mod case100_chainlink_config {
    use super::*;

    pub fn initialize_chainlink_config(ctx: Context<Initialize100>, param: u64) -> Result<()> {
        // Vulnerable initialization without init check
        let mut data = ChainlinkConfigData::try_from_slice(&ctx.accounts.data_account.data.borrow())?;
        data.value = param;  // Overwrites existing data
        data.serialize(&mut *ctx.accounts.data_account.data.borrow_mut())?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize100<'info> {
    #[account(mut)]
    pub data_account: AccountInfo<'info>,  // No init guard
    pub authority: Signer<'info>,
}

#[derive(BorshSerialize, BorshDeserialize)]
pub struct ChainlinkConfigData {
    pub value: u64,
}
