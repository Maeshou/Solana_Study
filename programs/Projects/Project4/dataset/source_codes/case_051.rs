use anchor_lang::prelude::*;
use borsh::{BorshDeserialize, BorshSerialize};

declare_id!("F00000000000000000000000000000033");

#[program]
pub mod case051_blacklist {
    use super::*;

    pub fn initialize_blacklist(ctx: Context<Initialize051>, param: u64) -> Result<()> {
        // Vulnerable initialization without init check
        let mut data = BlacklistData::try_from_slice(&ctx.accounts.data_account.data.borrow())?;
        data.value = param;  // Overwrites existing data
        data.serialize(&mut *ctx.accounts.data_account.data.borrow_mut())?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize051<'info> {
    #[account(mut)]
    pub data_account: AccountInfo<'info>,  // No init guard
    pub authority: Signer<'info>,
}

#[derive(BorshSerialize, BorshDeserialize)]
pub struct BlacklistData {
    pub value: u64,
}
