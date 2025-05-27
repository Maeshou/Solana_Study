use anchor_lang::prelude::*;
use borsh::{BorshDeserialize, BorshSerialize};

declare_id!("F0000000000000000000000000000000d");

#[program]
pub mod case013_lending_reserve {
    use super::*;

    pub fn initialize_lending_reserve(ctx: Context<Initialize013>, param: u64) -> Result<()> {
        // Vulnerable initialization without init check
        let mut data = LendingReserveData::try_from_slice(&ctx.accounts.data_account.data.borrow())?;
        data.value = param;  // Overwrites existing data
        data.serialize(&mut *ctx.accounts.data_account.data.borrow_mut())?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize013<'info> {
    #[account(mut)]
    pub data_account: AccountInfo<'info>,  // No init guard
    pub authority: Signer<'info>,
}

#[derive(BorshSerialize, BorshDeserialize)]
pub struct LendingReserveData {
    pub value: u64,
}
