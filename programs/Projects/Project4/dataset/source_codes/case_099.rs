use anchor_lang::prelude::*;
use borsh::{BorshDeserialize, BorshSerialize};

declare_id!("F00000000000000000000000000000063");

#[program]
pub mod case099_job_scheduler {
    use super::*;

    pub fn initialize_job_scheduler(ctx: Context<Initialize099>, param: u64) -> Result<()> {
        // Vulnerable initialization without init check
        let mut data = JobSchedulerData::try_from_slice(&ctx.accounts.data_account.data.borrow())?;
        data.value = param;  // Overwrites existing data
        data.serialize(&mut *ctx.accounts.data_account.data.borrow_mut())?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize099<'info> {
    #[account(mut)]
    pub data_account: AccountInfo<'info>,  // No init guard
    pub authority: Signer<'info>,
}

#[derive(BorshSerialize, BorshDeserialize)]
pub struct JobSchedulerData {
    pub value: u64,
}
