use anchor_lang::prelude::*;
use borsh::{BorshDeserialize, BorshSerialize};

declare_id!("F00000000000000000000000000000024");

#[program]
pub mod case036_treasury_account {
    use super::*;

    pub fn initialize_treasury_account(ctx: Context<Initialize036>, param: u64) -> Result<()> {
        // Vulnerable initialization without init check
        let mut data = TreasuryAccountData::try_from_slice(&ctx.accounts.data_account.data.borrow())?;
        data.value = param;  // Overwrites existing data
        data.serialize(&mut *ctx.accounts.data_account.data.borrow_mut())?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize036<'info> {
    #[account(mut)]
    pub data_account: AccountInfo<'info>,  // No init guard
    pub authority: Signer<'info>,
}

#[derive(BorshSerialize, BorshDeserialize)]
pub struct TreasuryAccountData {
    pub value: u64,
}
