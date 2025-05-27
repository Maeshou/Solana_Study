use anchor_lang::prelude::*;
use borsh::{BorshDeserialize, BorshSerialize};

declare_id!("F0000000000000000000000000000004d");

#[program]
pub mod case077_options_contract {
    use super::*;

    pub fn initialize_options_contract(ctx: Context<Initialize077>, param: u64) -> Result<()> {
        // Vulnerable initialization without init check
        let mut data = OptionsContractData::try_from_slice(&ctx.accounts.data_account.data.borrow())?;
        data.value = param;  // Overwrites existing data
        data.serialize(&mut *ctx.accounts.data_account.data.borrow_mut())?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize077<'info> {
    #[account(mut)]
    pub data_account: AccountInfo<'info>,  // No init guard
    pub authority: Signer<'info>,
}

#[derive(BorshSerialize, BorshDeserialize)]
pub struct OptionsContractData {
    pub value: u64,
}
