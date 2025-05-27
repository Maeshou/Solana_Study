use anchor_lang::prelude::*;
use borsh::{BorshDeserialize, BorshSerialize};

declare_id!("F0000000000000000000000000000000f");

#[program]
pub mod case015_oracle_config {
    use super::*;

    pub fn initialize_oracle_config(ctx: Context<Initialize015>, param: u64) -> Result<()> {
        // Vulnerable initialization without init check
        let mut data = OracleConfigData::try_from_slice(&ctx.accounts.data_account.data.borrow())?;
        data.value = param;  // Overwrites existing data
        data.serialize(&mut *ctx.accounts.data_account.data.borrow_mut())?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize015<'info> {
    #[account(mut)]
    pub data_account: AccountInfo<'info>,  // No init guard
    pub authority: Signer<'info>,
}

#[derive(BorshSerialize, BorshDeserialize)]
pub struct OracleConfigData {
    pub value: u64,
}
