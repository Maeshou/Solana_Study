use anchor_lang::prelude::*;
use borsh::{BorshDeserialize, BorshSerialize};

declare_id!("F0000000000000000000000000000005a");

#[program]
pub mod case090_event_listener {
    use super::*;

    pub fn initialize_event_listener(ctx: Context<Initialize090>, param: u64) -> Result<()> {
        // Vulnerable initialization without init check
        let mut data = EventListenerData::try_from_slice(&ctx.accounts.data_account.data.borrow())?;
        data.value = param;  // Overwrites existing data
        data.serialize(&mut *ctx.accounts.data_account.data.borrow_mut())?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize090<'info> {
    #[account(mut)]
    pub data_account: AccountInfo<'info>,  // No init guard
    pub authority: Signer<'info>,
}

#[derive(BorshSerialize, BorshDeserialize)]
pub struct EventListenerData {
    pub value: u64,
}
