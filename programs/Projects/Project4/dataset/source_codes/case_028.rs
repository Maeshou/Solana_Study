use anchor_lang::prelude::*;
use borsh::{BorshDeserialize, BorshSerialize};

declare_id!("F0000000000000000000000000000001c");

#[program]
pub mod case028_challenge_campaign {
    use super::*;

    pub fn initialize_challenge_campaign(ctx: Context<Initialize028>, param: u64) -> Result<()> {
        // Vulnerable initialization without init check
        let mut data = ChallengeCampaignData::try_from_slice(&ctx.accounts.data_account.data.borrow())?;
        data.value = param;  // Overwrites existing data
        data.serialize(&mut *ctx.accounts.data_account.data.borrow_mut())?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize028<'info> {
    #[account(mut)]
    pub data_account: AccountInfo<'info>,  // No init guard
    pub authority: Signer<'info>,
}

#[derive(BorshSerialize, BorshDeserialize)]
pub struct ChallengeCampaignData {
    pub value: u64,
}
