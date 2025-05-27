use anchor_lang::prelude::*;
use borsh::{BorshDeserialize, BorshSerialize};

declare_id!("F0000000000000000000000000000000c");

#[program]
pub mod case012_liquidity_pool {
    use super::*;

    pub fn initialize_liquidity_pool(ctx: Context<Initialize012>, param: u64) -> Result<()> {
        // Vulnerable initialization without init check
        let mut data = LiquidityPoolData::try_from_slice(&ctx.accounts.data_account.data.borrow())?;
        data.value = param;  // Overwrites existing data
        data.serialize(&mut *ctx.accounts.data_account.data.borrow_mut())?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize012<'info> {
    #[account(mut)]
    pub data_account: AccountInfo<'info>,  // No init guard
    pub authority: Signer<'info>,
}

#[derive(BorshSerialize, BorshDeserialize)]
pub struct LiquidityPoolData {
    pub value: u64,
}
