use anchor_lang::prelude::*;
use borsh::{BorshDeserialize, BorshSerialize};

declare_id!("F00000000000000000000000000000038");

#[program]
pub mod case056_lockup_schedule {
    use super::*;

    pub fn initialize_lockup_schedule(ctx: Context<Initialize056>, param: u64) -> Result<()> {
        // Vulnerable initialization without init check
        let mut data = LockupScheduleData::try_from_slice(&ctx.accounts.data_account.data.borrow())?;
        data.value = param;  // Overwrites existing data
        data.serialize(&mut *ctx.accounts.data_account.data.borrow_mut())?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize056<'info> {
    #[account(mut)]
    pub data_account: AccountInfo<'info>,  // No init guard
    pub authority: Signer<'info>,
}

#[derive(BorshSerialize, BorshDeserialize)]
pub struct LockupScheduleData {
    pub value: u64,
}
