use anchor_lang::prelude::*;
use borsh::{BorshDeserialize, BorshSerialize};

declare_id!("F00000000000000000000000000000045");

#[program]
pub mod case069_derivatives_contract {
    use super::*;

    pub fn initialize_derivatives_contract(ctx: Context<Initialize069>, param: u64) -> Result<()> {
        // Vulnerable initialization without init check
        let mut data = DerivativesContractData::try_from_slice(&ctx.accounts.data_account.data.borrow())?;
        data.value = param;  // Overwrites existing data
        data.serialize(&mut *ctx.accounts.data_account.data.borrow_mut())?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize069<'info> {
    #[account(mut)]
    pub data_account: AccountInfo<'info>,  // No init guard
    pub authority: Signer<'info>,
}

#[derive(BorshSerialize, BorshDeserialize)]
pub struct DerivativesContractData {
    pub value: u64,
}
