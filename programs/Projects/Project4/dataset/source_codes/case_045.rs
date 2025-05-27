use anchor_lang::prelude::*;
use borsh::{BorshDeserialize, BorshSerialize};

declare_id!("F0000000000000000000000000000002d");

#[program]
pub mod case045_vault_account {
    use super::*;

    pub fn initialize_vault_account(ctx: Context<Initialize045>, param: u64) -> Result<()> {
        // Vulnerable initialization without init check
        let mut data = VaultAccountData::try_from_slice(&ctx.accounts.data_account.data.borrow())?;
        data.value = param;  // Overwrites existing data
        data.serialize(&mut *ctx.accounts.data_account.data.borrow_mut())?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize045<'info> {
    #[account(mut)]
    pub data_account: AccountInfo<'info>,  // No init guard
    pub authority: Signer<'info>,
}

#[derive(BorshSerialize, BorshDeserialize)]
pub struct VaultAccountData {
    pub value: u64,
}
