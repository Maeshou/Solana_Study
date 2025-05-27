use anchor_lang::prelude::*;
use borsh::{BorshDeserialize, BorshSerialize};

declare_id!("F00000000000000000000000000000034");

#[program]
pub mod case052_whitelist {
    use super::*;

    pub fn initialize_whitelist(ctx: Context<Initialize052>, param: u64) -> Result<()> {
        // Vulnerable initialization without init check
        let mut data = WhitelistData::try_from_slice(&ctx.accounts.data_account.data.borrow())?;
        data.value = param;  // Overwrites existing data
        data.serialize(&mut *ctx.accounts.data_account.data.borrow_mut())?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize052<'info> {
    #[account(mut)]
    pub data_account: AccountInfo<'info>,  // No init guard
    pub authority: Signer<'info>,
}

#[derive(BorshSerialize, BorshDeserialize)]
pub struct WhitelistData {
    pub value: u64,
}
