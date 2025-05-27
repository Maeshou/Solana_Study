use anchor_lang::prelude::*;
use borsh::{BorshDeserialize, BorshSerialize};

declare_id!("F00000000000000000000000000000055");

#[program]
pub mod case085_oracle_feed {
    use super::*;

    pub fn initialize_oracle_feed(ctx: Context<Initialize085>, param: u64) -> Result<()> {
        // Vulnerable initialization without init check
        let mut data = OracleFeedData::try_from_slice(&ctx.accounts.data_account.data.borrow())?;
        data.value = param;  // Overwrites existing data
        data.serialize(&mut *ctx.accounts.data_account.data.borrow_mut())?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize085<'info> {
    #[account(mut)]
    pub data_account: AccountInfo<'info>,  // No init guard
    pub authority: Signer<'info>,
}

#[derive(BorshSerialize, BorshDeserialize)]
pub struct OracleFeedData {
    pub value: u64,
}
