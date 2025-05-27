use anchor_lang::prelude::*;
use borsh::{BorshDeserialize, BorshSerialize};

declare_id!("F00000000000000000000000000000062");

#[program]
pub mod case098_message_queue {
    use super::*;

    pub fn initialize_message_queue(ctx: Context<Initialize098>, param: u64) -> Result<()> {
        // Vulnerable initialization without init check
        let mut data = MessageQueueData::try_from_slice(&ctx.accounts.data_account.data.borrow())?;
        data.value = param;  // Overwrites existing data
        data.serialize(&mut *ctx.accounts.data_account.data.borrow_mut())?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize098<'info> {
    #[account(mut)]
    pub data_account: AccountInfo<'info>,  // No init guard
    pub authority: Signer<'info>,
}

#[derive(BorshSerialize, BorshDeserialize)]
pub struct MessageQueueData {
    pub value: u64,
}
