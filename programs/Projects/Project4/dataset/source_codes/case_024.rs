use anchor_lang::prelude::*;
use borsh::{BorshDeserialize, BorshSerialize};

declare_id!("F00000000000000000000000000000018");

#[program]
pub mod case024_price_feed {
    use super::*;

    pub fn initialize_price_feed(ctx: Context<Initialize024>, param: u64) -> Result<()> {
        // Vulnerable initialization without init check
        let mut data = PriceFeedData::try_from_slice(&ctx.accounts.data_account.data.borrow())?;
        data.value = param;  // Overwrites existing data
        data.serialize(&mut *ctx.accounts.data_account.data.borrow_mut())?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize024<'info> {
    #[account(mut)]
    pub data_account: AccountInfo<'info>,  // No init guard
    pub authority: Signer<'info>,
}

#[derive(BorshSerialize, BorshDeserialize)]
pub struct PriceFeedData {
    pub value: u64,
}
