use anchor_lang::prelude::*;
use borsh::{BorshDeserialize, BorshSerialize};

declare_id!("F00000000000000000000000000000030");

#[program]
pub mod case048_rate_limit {
    use super::*;

    pub fn initialize_rate_limit(ctx: Context<Initialize048>, param: u64) -> Result<()> {
        // Vulnerable initialization without init check
        let mut data = RateLimitData::try_from_slice(&ctx.accounts.data_account.data.borrow())?;
        data.value = param;  // Overwrites existing data
        data.serialize(&mut *ctx.accounts.data_account.data.borrow_mut())?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize048<'info> {
    #[account(mut)]
    pub data_account: AccountInfo<'info>,  // No init guard
    pub authority: Signer<'info>,
}

#[derive(BorshSerialize, BorshDeserialize)]
pub struct RateLimitData {
    pub value: u64,
}
