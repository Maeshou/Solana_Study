use anchor_lang::prelude::*;
use borsh::{BorshDeserialize, BorshSerialize};

declare_id!("F00000000000000000000000000000003");

#[program]
pub mod case003_bounty_campaign {
    use super::*;

    pub fn initialize_bounty_campaign(ctx: Context<Initialize003>, param: u64) -> Result<()> {
        // Vulnerable initialization without init check
        let mut data = BountyCampaignData::try_from_slice(&ctx.accounts.data_account.data.borrow())?;
        data.value = param;  // Overwrites existing data
        data.serialize(&mut *ctx.accounts.data_account.data.borrow_mut())?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize003<'info> {
    #[account(mut)]
    pub data_account: AccountInfo<'info>,  // No init guard
    pub authority: Signer<'info>,
}

#[derive(BorshSerialize, BorshDeserialize)]
pub struct BountyCampaignData {
    pub value: u64,
}
