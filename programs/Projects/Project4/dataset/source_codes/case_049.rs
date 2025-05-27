use anchor_lang::prelude::*;
use borsh::{BorshDeserialize, BorshSerialize};

declare_id!("F00000000000000000000000000000031");

#[program]
pub mod case049_kyc_record {
    use super::*;

    pub fn initialize_kyc_record(ctx: Context<Initialize049>, param: u64) -> Result<()> {
        // Vulnerable initialization without init check
        let mut data = KycRecordData::try_from_slice(&ctx.accounts.data_account.data.borrow())?;
        data.value = param;  // Overwrites existing data
        data.serialize(&mut *ctx.accounts.data_account.data.borrow_mut())?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize049<'info> {
    #[account(mut)]
    pub data_account: AccountInfo<'info>,  // No init guard
    pub authority: Signer<'info>,
}

#[derive(BorshSerialize, BorshDeserialize)]
pub struct KycRecordData {
    pub value: u64,
}
