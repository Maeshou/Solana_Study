use anchor_lang::prelude::*;
use borsh::{BorshDeserialize, BorshSerialize};

declare_id!("F00000000000000000000000000000029");

#[program]
pub mod case041_closing_state {
    use super::*;

    pub fn initialize_closing_state(ctx: Context<Initialize041>, param: u64) -> Result<()> {
        // Vulnerable initialization without init check
        let mut data = ClosingStateData::try_from_slice(&ctx.accounts.data_account.data.borrow())?;
        data.value = param;  // Overwrites existing data
        data.serialize(&mut *ctx.accounts.data_account.data.borrow_mut())?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize041<'info> {
    #[account(mut)]
    pub data_account: AccountInfo<'info>,  // No init guard
    pub authority: Signer<'info>,
}

#[derive(BorshSerialize, BorshDeserialize)]
pub struct ClosingStateData {
    pub value: u64,
}
