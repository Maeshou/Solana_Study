use anchor_lang::prelude::*;
use borsh::{BorshDeserialize, BorshSerialize};

declare_id!("F00000000000000000000000000000040");

#[program]
pub mod case064_masternode_control {
    use super::*;

    pub fn initialize_masternode_control(ctx: Context<Initialize064>, param: u64) -> Result<()> {
        // Vulnerable initialization without init check
        let mut data = MasternodeControlData::try_from_slice(&ctx.accounts.data_account.data.borrow())?;
        data.value = param;  // Overwrites existing data
        data.serialize(&mut *ctx.accounts.data_account.data.borrow_mut())?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize064<'info> {
    #[account(mut)]
    pub data_account: AccountInfo<'info>,  // No init guard
    pub authority: Signer<'info>,
}

#[derive(BorshSerialize, BorshDeserialize)]
pub struct MasternodeControlData {
    pub value: u64,
}
