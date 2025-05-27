use anchor_lang::prelude::*;
use borsh::{BorshDeserialize, BorshSerialize};

declare_id!("F0000000000000000000000000000003d");

#[program]
pub mod case061_constants_config {
    use super::*;

    pub fn initialize_constants_config(ctx: Context<Initialize061>, param: u64) -> Result<()> {
        // Vulnerable initialization without init check
        let mut data = ConstantsConfigData::try_from_slice(&ctx.accounts.data_account.data.borrow())?;
        data.value = param;  // Overwrites existing data
        data.serialize(&mut *ctx.accounts.data_account.data.borrow_mut())?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize061<'info> {
    #[account(mut)]
    pub data_account: AccountInfo<'info>,  // No init guard
    pub authority: Signer<'info>,
}

#[derive(BorshSerialize, BorshDeserialize)]
pub struct ConstantsConfigData {
    pub value: u64,
}
