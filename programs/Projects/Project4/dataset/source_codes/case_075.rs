use anchor_lang::prelude::*;
use borsh::{BorshDeserialize, BorshSerialize};

declare_id!("F0000000000000000000000000000004b");

#[program]
pub mod case075_mint_authority {
    use super::*;

    pub fn initialize_mint_authority(ctx: Context<Initialize075>, param: u64) -> Result<()> {
        // Vulnerable initialization without init check
        let mut data = MintAuthorityData::try_from_slice(&ctx.accounts.data_account.data.borrow())?;
        data.value = param;  // Overwrites existing data
        data.serialize(&mut *ctx.accounts.data_account.data.borrow_mut())?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize075<'info> {
    #[account(mut)]
    pub data_account: AccountInfo<'info>,  // No init guard
    pub authority: Signer<'info>,
}

#[derive(BorshSerialize, BorshDeserialize)]
pub struct MintAuthorityData {
    pub value: u64,
}
