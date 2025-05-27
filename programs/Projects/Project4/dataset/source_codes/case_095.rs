use anchor_lang::prelude::*;
use borsh::{BorshDeserialize, BorshSerialize};

declare_id!("F0000000000000000000000000000005f");

#[program]
pub mod case095_gas_control {
    use super::*;

    pub fn initialize_gas_control(ctx: Context<Initialize095>, param: u64) -> Result<()> {
        // Vulnerable initialization without init check
        let mut data = GasControlData::try_from_slice(&ctx.accounts.data_account.data.borrow())?;
        data.value = param;  // Overwrites existing data
        data.serialize(&mut *ctx.accounts.data_account.data.borrow_mut())?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize095<'info> {
    #[account(mut)]
    pub data_account: AccountInfo<'info>,  // No init guard
    pub authority: Signer<'info>,
}

#[derive(BorshSerialize, BorshDeserialize)]
pub struct GasControlData {
    pub value: u64,
}
