use anchor_lang::prelude::*;
use borsh::{BorshDeserialize, BorshSerialize};

declare_id!("F0000000000000000000000000000002a");

#[program]
pub mod case042_liquidation_account {
    use super::*;

    pub fn initialize_liquidation_account(ctx: Context<Initialize042>, param: u64) -> Result<()> {
        // Vulnerable initialization without init check
        let mut data = LiquidationAccountData::try_from_slice(&ctx.accounts.data_account.data.borrow())?;
        data.value = param;  // Overwrites existing data
        data.serialize(&mut *ctx.accounts.data_account.data.borrow_mut())?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize042<'info> {
    #[account(mut)]
    pub data_account: AccountInfo<'info>,  // No init guard
    pub authority: Signer<'info>,
}

#[derive(BorshSerialize, BorshDeserialize)]
pub struct LiquidationAccountData {
    pub value: u64,
}
