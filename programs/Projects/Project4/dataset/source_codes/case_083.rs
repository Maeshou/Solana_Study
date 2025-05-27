use anchor_lang::prelude::*;
use borsh::{BorshDeserialize, BorshSerialize};

declare_id!("F00000000000000000000000000000053");

#[program]
pub mod case083_commission_settings {
    use super::*;

    pub fn initialize_commission_settings(ctx: Context<Initialize083>, param: u64) -> Result<()> {
        // Vulnerable initialization without init check
        let mut data = CommissionSettingsData::try_from_slice(&ctx.accounts.data_account.data.borrow())?;
        data.value = param;  // Overwrites existing data
        data.serialize(&mut *ctx.accounts.data_account.data.borrow_mut())?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize083<'info> {
    #[account(mut)]
    pub data_account: AccountInfo<'info>,  // No init guard
    pub authority: Signer<'info>,
}

#[derive(BorshSerialize, BorshDeserialize)]
pub struct CommissionSettingsData {
    pub value: u64,
}
