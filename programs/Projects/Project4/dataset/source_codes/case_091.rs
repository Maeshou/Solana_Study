use anchor_lang::prelude::*;
use borsh::{BorshDeserialize, BorshSerialize};

declare_id!("F0000000000000000000000000000005b");

#[program]
pub mod case091_notifier_system {
    use super::*;

    pub fn initialize_notifier_system(ctx: Context<Initialize091>, param: u64) -> Result<()> {
        // Vulnerable initialization without init check
        let mut data = NotifierSystemData::try_from_slice(&ctx.accounts.data_account.data.borrow())?;
        data.value = param;  // Overwrites existing data
        data.serialize(&mut *ctx.accounts.data_account.data.borrow_mut())?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize091<'info> {
    #[account(mut)]
    pub data_account: AccountInfo<'info>,  // No init guard
    pub authority: Signer<'info>,
}

#[derive(BorshSerialize, BorshDeserialize)]
pub struct NotifierSystemData {
    pub value: u64,
}
