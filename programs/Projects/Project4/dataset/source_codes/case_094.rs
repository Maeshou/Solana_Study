use anchor_lang::prelude::*;
use borsh::{BorshDeserialize, BorshSerialize};

declare_id!("F0000000000000000000000000000005e");

#[program]
pub mod case094_frontrun_guard {
    use super::*;

    pub fn initialize_frontrun_guard(ctx: Context<Initialize094>, param: u64) -> Result<()> {
        // Vulnerable initialization without init check
        let mut data = FrontrunGuardData::try_from_slice(&ctx.accounts.data_account.data.borrow())?;
        data.value = param;  // Overwrites existing data
        data.serialize(&mut *ctx.accounts.data_account.data.borrow_mut())?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize094<'info> {
    #[account(mut)]
    pub data_account: AccountInfo<'info>,  // No init guard
    pub authority: Signer<'info>,
}

#[derive(BorshSerialize, BorshDeserialize)]
pub struct FrontrunGuardData {
    pub value: u64,
}
