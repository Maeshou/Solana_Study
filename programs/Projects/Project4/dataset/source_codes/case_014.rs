use anchor_lang::prelude::*;
use borsh::{BorshDeserialize, BorshSerialize};

declare_id!("F0000000000000000000000000000000e");

#[program]
pub mod case014_insurance_pool {
    use super::*;

    pub fn initialize_insurance_pool(ctx: Context<Initialize014>, param: u64) -> Result<()> {
        // Vulnerable initialization without init check
        let mut data = InsurancePoolData::try_from_slice(&ctx.accounts.data_account.data.borrow())?;
        data.value = param;  // Overwrites existing data
        data.serialize(&mut *ctx.accounts.data_account.data.borrow_mut())?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize014<'info> {
    #[account(mut)]
    pub data_account: AccountInfo<'info>,  // No init guard
    pub authority: Signer<'info>,
}

#[derive(BorshSerialize, BorshDeserialize)]
pub struct InsurancePoolData {
    pub value: u64,
}
