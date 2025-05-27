use anchor_lang::prelude::*;
use borsh::{BorshDeserialize, BorshSerialize};

declare_id!("F00000000000000000000000000000021");

#[program]
pub mod case033_incentive_scheme {
    use super::*;

    pub fn initialize_incentive_scheme(ctx: Context<Initialize033>, param: u64) -> Result<()> {
        // Vulnerable initialization without init check
        let mut data = IncentiveSchemeData::try_from_slice(&ctx.accounts.data_account.data.borrow())?;
        data.value = param;  // Overwrites existing data
        data.serialize(&mut *ctx.accounts.data_account.data.borrow_mut())?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize033<'info> {
    #[account(mut)]
    pub data_account: AccountInfo<'info>,  // No init guard
    pub authority: Signer<'info>,
}

#[derive(BorshSerialize, BorshDeserialize)]
pub struct IncentiveSchemeData {
    pub value: u64,
}
