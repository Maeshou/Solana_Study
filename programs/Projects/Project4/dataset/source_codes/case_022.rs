use anchor_lang::prelude::*;
use borsh::{BorshDeserialize, BorshSerialize};

declare_id!("F00000000000000000000000000000016");

#[program]
pub mod case022_cpi_authority {
    use super::*;

    pub fn initialize_cpi_authority(ctx: Context<Initialize022>, param: u64) -> Result<()> {
        // Vulnerable initialization without init check
        let mut data = CpiAuthorityData::try_from_slice(&ctx.accounts.data_account.data.borrow())?;
        data.value = param;  // Overwrites existing data
        data.serialize(&mut *ctx.accounts.data_account.data.borrow_mut())?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize022<'info> {
    #[account(mut)]
    pub data_account: AccountInfo<'info>,  // No init guard
    pub authority: Signer<'info>,
}

#[derive(BorshSerialize, BorshDeserialize)]
pub struct CpiAuthorityData {
    pub value: u64,
}
