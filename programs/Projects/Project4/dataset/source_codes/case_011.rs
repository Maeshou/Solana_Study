use anchor_lang::prelude::*;
use borsh::{BorshDeserialize, BorshSerialize};

declare_id!("F0000000000000000000000000000000b");

#[program]
pub mod case011_mint_settings {
    use super::*;

    pub fn initialize_mint_settings(ctx: Context<Initialize011>, param: u64) -> Result<()> {
        // Vulnerable initialization without init check
        let mut data = MintSettingsData::try_from_slice(&ctx.accounts.data_account.data.borrow())?;
        data.value = param;  // Overwrites existing data
        data.serialize(&mut *ctx.accounts.data_account.data.borrow_mut())?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize011<'info> {
    #[account(mut)]
    pub data_account: AccountInfo<'info>,  // No init guard
    pub authority: Signer<'info>,
}

#[derive(BorshSerialize, BorshDeserialize)]
pub struct MintSettingsData {
    pub value: u64,
}
