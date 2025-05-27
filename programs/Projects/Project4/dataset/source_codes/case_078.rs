use anchor_lang::prelude::*;
use borsh::{BorshDeserialize, BorshSerialize};

declare_id!("F0000000000000000000000000000004e");

#[program]
pub mod case078_futures_contract {
    use super::*;

    pub fn initialize_futures_contract(ctx: Context<Initialize078>, param: u64) -> Result<()> {
        // Vulnerable initialization without init check
        let mut data = FuturesContractData::try_from_slice(&ctx.accounts.data_account.data.borrow())?;
        data.value = param;  // Overwrites existing data
        data.serialize(&mut *ctx.accounts.data_account.data.borrow_mut())?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize078<'info> {
    #[account(mut)]
    pub data_account: AccountInfo<'info>,  // No init guard
    pub authority: Signer<'info>,
}

#[derive(BorshSerialize, BorshDeserialize)]
pub struct FuturesContractData {
    pub value: u64,
}
