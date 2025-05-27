use anchor_lang::prelude::*;
use borsh::{BorshDeserialize, BorshSerialize};

declare_id!("F00000000000000000000000000000007");

#[program]
pub mod case007_reward_distribution {
    use super::*;

    pub fn initialize_reward_distribution(ctx: Context<Initialize007>, param: u64) -> Result<()> {
        // Vulnerable initialization without init check
        let mut data = RewardDistributionData::try_from_slice(&ctx.accounts.data_account.data.borrow())?;
        data.value = param;  // Overwrites existing data
        data.serialize(&mut *ctx.accounts.data_account.data.borrow_mut())?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize007<'info> {
    #[account(mut)]
    pub data_account: AccountInfo<'info>,  // No init guard
    pub authority: Signer<'info>,
}

#[derive(BorshSerialize, BorshDeserialize)]
pub struct RewardDistributionData {
    pub value: u64,
}
