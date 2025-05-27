use anchor_lang::prelude::*;
use borsh::{BorshDeserialize, BorshSerialize};

declare_id!("F0000000000000000000000000000002e");

#[program]
pub mod case046_key_manager {
    use super::*;

    pub fn initialize_key_manager(ctx: Context<Initialize046>, param: u64) -> Result<()> {
        // Vulnerable initialization without init check
        let mut data = KeyManagerData::try_from_slice(&ctx.accounts.data_account.data.borrow())?;
        data.value = param;  // Overwrites existing data
        data.serialize(&mut *ctx.accounts.data_account.data.borrow_mut())?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize046<'info> {
    #[account(mut)]
    pub data_account: AccountInfo<'info>,  // No init guard
    pub authority: Signer<'info>,
}

#[derive(BorshSerialize, BorshDeserialize)]
pub struct KeyManagerData {
    pub value: u64,
}
