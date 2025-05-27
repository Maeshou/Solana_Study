use anchor_lang::prelude::*;
use borsh::{BorshDeserialize, BorshSerialize};

declare_id!("F00000000000000000000000000000006");

#[program]
pub mod case006_stake_pool {
    use super::*;

    pub fn initialize_stake_pool(ctx: Context<Initialize006>, param: u64) -> Result<()> {
        // Vulnerable initialization without init check
        let mut data = StakePoolData::try_from_slice(&ctx.accounts.data_account.data.borrow())?;
        data.value = param;  // Overwrites existing data
        data.serialize(&mut *ctx.accounts.data_account.data.borrow_mut())?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize006<'info> {
    #[account(mut)]
    pub data_account: AccountInfo<'info>,  // No init guard
    pub authority: Signer<'info>,
}

#[derive(BorshSerialize, BorshDeserialize)]
pub struct StakePoolData {
    pub value: u64,
}
