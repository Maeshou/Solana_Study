use anchor_lang::prelude::*;
use borsh::{BorshDeserialize, BorshSerialize};

declare_id!("F00000000000000000000000000000009");

#[program]
pub mod case009_vote_record {
    use super::*;

    pub fn initialize_vote_record(ctx: Context<Initialize009>, param: u64) -> Result<()> {
        // Vulnerable initialization without init check
        let mut data = VoteRecordData::try_from_slice(&ctx.accounts.data_account.data.borrow())?;
        data.value = param;  // Overwrites existing data
        data.serialize(&mut *ctx.accounts.data_account.data.borrow_mut())?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize009<'info> {
    #[account(mut)]
    pub data_account: AccountInfo<'info>,  // No init guard
    pub authority: Signer<'info>,
}

#[derive(BorshSerialize, BorshDeserialize)]
pub struct VoteRecordData {
    pub value: u64,
}
