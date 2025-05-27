use anchor_lang::prelude::*;
use borsh::{BorshDeserialize, BorshSerialize};

declare_id!("F00000000000000000000000000000015");

#[program]
pub mod case021_content_license {
    use super::*;

    pub fn initialize_content_license(ctx: Context<Initialize021>, param: u64) -> Result<()> {
        // Vulnerable initialization without init check
        let mut data = ContentLicenseData::try_from_slice(&ctx.accounts.data_account.data.borrow())?;
        data.value = param;  // Overwrites existing data
        data.serialize(&mut *ctx.accounts.data_account.data.borrow_mut())?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize021<'info> {
    #[account(mut)]
    pub data_account: AccountInfo<'info>,  // No init guard
    pub authority: Signer<'info>,
}

#[derive(BorshSerialize, BorshDeserialize)]
pub struct ContentLicenseData {
    pub value: u64,
}
