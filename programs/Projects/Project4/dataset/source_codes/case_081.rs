use anchor_lang::prelude::*;
use borsh::{BorshDeserialize, BorshSerialize};

declare_id!("F00000000000000000000000000000051");

#[program]
pub mod case081_community_fund {
    use super::*;

    pub fn initialize_community_fund(ctx: Context<Initialize081>, param: u64) -> Result<()> {
        // Vulnerable initialization without init check
        let mut data = CommunityFundData::try_from_slice(&ctx.accounts.data_account.data.borrow())?;
        data.value = param;  // Overwrites existing data
        data.serialize(&mut *ctx.accounts.data_account.data.borrow_mut())?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize081<'info> {
    #[account(mut)]
    pub data_account: AccountInfo<'info>,  // No init guard
    pub authority: Signer<'info>,
}

#[derive(BorshSerialize, BorshDeserialize)]
pub struct CommunityFundData {
    pub value: u64,
}
