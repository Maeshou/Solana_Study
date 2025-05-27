use anchor_lang::prelude::*;
use borsh::{BorshDeserialize, BorshSerialize};

declare_id!("F00000000000000000000000000000050");

#[program]
pub mod case080_dao_treasury {
    use super::*;

    pub fn initialize_dao_treasury(ctx: Context<Initialize080>, param: u64) -> Result<()> {
        // Vulnerable initialization without init check
        let mut data = DaoTreasuryData::try_from_slice(&ctx.accounts.data_account.data.borrow())?;
        data.value = param;  // Overwrites existing data
        data.serialize(&mut *ctx.accounts.data_account.data.borrow_mut())?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize080<'info> {
    #[account(mut)]
    pub data_account: AccountInfo<'info>,  // No init guard
    pub authority: Signer<'info>,
}

#[derive(BorshSerialize, BorshDeserialize)]
pub struct DaoTreasuryData {
    pub value: u64,
}
