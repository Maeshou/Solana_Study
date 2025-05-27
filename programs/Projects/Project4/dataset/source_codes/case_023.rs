use anchor_lang::prelude::*;
use borsh::{BorshDeserialize, BorshSerialize};

declare_id!("F00000000000000000000000000000017");

#[program]
pub mod case023_data_oracle {
    use super::*;

    pub fn initialize_data_oracle(ctx: Context<Initialize023>, param: u64) -> Result<()> {
        // Vulnerable initialization without init check
        let mut data = DataOracleData::try_from_slice(&ctx.accounts.data_account.data.borrow())?;
        data.value = param;  // Overwrites existing data
        data.serialize(&mut *ctx.accounts.data_account.data.borrow_mut())?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize023<'info> {
    #[account(mut)]
    pub data_account: AccountInfo<'info>,  // No init guard
    pub authority: Signer<'info>,
}

#[derive(BorshSerialize, BorshDeserialize)]
pub struct DataOracleData {
    pub value: u64,
}
