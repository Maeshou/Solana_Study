use anchor_lang::prelude::*;
use borsh::{BorshDeserialize, BorshSerialize};

declare_id!("F0000000000000000000000000000001e");

#[program]
pub mod case030_donation_pool {
    use super::*;

    pub fn initialize_donation_pool(ctx: Context<Initialize030>, param: u64) -> Result<()> {
        // Vulnerable initialization without init check
        let mut data = DonationPoolData::try_from_slice(&ctx.accounts.data_account.data.borrow())?;
        data.value = param;  // Overwrites existing data
        data.serialize(&mut *ctx.accounts.data_account.data.borrow_mut())?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize030<'info> {
    #[account(mut)]
    pub data_account: AccountInfo<'info>,  // No init guard
    pub authority: Signer<'info>,
}

#[derive(BorshSerialize, BorshDeserialize)]
pub struct DonationPoolData {
    pub value: u64,
}
