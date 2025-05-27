use anchor_lang::prelude::*;
use borsh::{BorshDeserialize, BorshSerialize};

declare_id!("F0000000000000000000000000000001b");

#[program]
pub mod case027_reward_pool {
    use super::*;

    pub fn initialize_reward_pool(ctx: Context<Initialize027>, param: u64) -> Result<()> {
        // Vulnerable initialization without init check
        let mut data = RewardPoolData::try_from_slice(&ctx.accounts.data_account.data.borrow())?;
        data.value = param;  // Overwrites existing data
        data.serialize(&mut *ctx.accounts.data_account.data.borrow_mut())?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize027<'info> {
    #[account(mut)]
    pub data_account: AccountInfo<'info>,  // No init guard
    pub authority: Signer<'info>,
}

#[derive(BorshSerialize, BorshDeserialize)]
pub struct RewardPoolData {
    pub value: u64,
}
