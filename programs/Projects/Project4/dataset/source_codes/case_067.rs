use anchor_lang::prelude::*;
use borsh::{BorshDeserialize, BorshSerialize};

declare_id!("F00000000000000000000000000000043");

#[program]
pub mod case067_revenue_share {
    use super::*;

    pub fn initialize_revenue_share(ctx: Context<Initialize067>, param: u64) -> Result<()> {
        // Vulnerable initialization without init check
        let mut data = RevenueShareData::try_from_slice(&ctx.accounts.data_account.data.borrow())?;
        data.value = param;  // Overwrites existing data
        data.serialize(&mut *ctx.accounts.data_account.data.borrow_mut())?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize067<'info> {
    #[account(mut)]
    pub data_account: AccountInfo<'info>,  // No init guard
    pub authority: Signer<'info>,
}

#[derive(BorshSerialize, BorshDeserialize)]
pub struct RevenueShareData {
    pub value: u64,
}
