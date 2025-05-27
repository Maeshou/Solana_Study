use anchor_lang::prelude::*;
use borsh::{BorshDeserialize, BorshSerialize};

declare_id!("F00000000000000000000000000000052");

#[program]
pub mod case082_developer_rewards {
    use super::*;

    pub fn initialize_developer_rewards(ctx: Context<Initialize082>, param: u64) -> Result<()> {
        // Vulnerable initialization without init check
        let mut data = DeveloperRewardsData::try_from_slice(&ctx.accounts.data_account.data.borrow())?;
        data.value = param;  // Overwrites existing data
        data.serialize(&mut *ctx.accounts.data_account.data.borrow_mut())?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize082<'info> {
    #[account(mut)]
    pub data_account: AccountInfo<'info>,  // No init guard
    pub authority: Signer<'info>,
}

#[derive(BorshSerialize, BorshDeserialize)]
pub struct DeveloperRewardsData {
    pub value: u64,
}
