use anchor_lang::prelude::*;
use borsh::{BorshDeserialize, BorshSerialize};

declare_id!("F00000000000000000000000000000058");

#[program]
pub mod case088_access_log {
    use super::*;

    pub fn initialize_access_log(ctx: Context<Initialize088>, param: u64) -> Result<()> {
        // Vulnerable initialization without init check
        let mut data = AccessLogData::try_from_slice(&ctx.accounts.data_account.data.borrow())?;
        data.value = param;  // Overwrites existing data
        data.serialize(&mut *ctx.accounts.data_account.data.borrow_mut())?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize088<'info> {
    #[account(mut)]
    pub data_account: AccountInfo<'info>,  // No init guard
    pub authority: Signer<'info>,
}

#[derive(BorshSerialize, BorshDeserialize)]
pub struct AccessLogData {
    pub value: u64,
}
