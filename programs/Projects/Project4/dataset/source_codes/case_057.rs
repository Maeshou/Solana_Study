use anchor_lang::prelude::*;
use borsh::{BorshDeserialize, BorshSerialize};

declare_id!("F00000000000000000000000000000039");

#[program]
pub mod case057_price_ceiling {
    use super::*;

    pub fn initialize_price_ceiling(ctx: Context<Initialize057>, param: u64) -> Result<()> {
        // Vulnerable initialization without init check
        let mut data = PriceCeilingData::try_from_slice(&ctx.accounts.data_account.data.borrow())?;
        data.value = param;  // Overwrites existing data
        data.serialize(&mut *ctx.accounts.data_account.data.borrow_mut())?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize057<'info> {
    #[account(mut)]
    pub data_account: AccountInfo<'info>,  // No init guard
    pub authority: Signer<'info>,
}

#[derive(BorshSerialize, BorshDeserialize)]
pub struct PriceCeilingData {
    pub value: u64,
}
