use anchor_lang::prelude::*;
use borsh::{BorshDeserialize, BorshSerialize};

declare_id!("F00000000000000000000000000000004");

#[program]
pub mod case004_escrow_account {
    use super::*;

    pub fn initialize_escrow_account(ctx: Context<Initialize004>, param: u64) -> Result<()> {
        // Vulnerable initialization without init check
        let mut data = EscrowAccountData::try_from_slice(&ctx.accounts.data_account.data.borrow())?;
        data.value = param;  // Overwrites existing data
        data.serialize(&mut *ctx.accounts.data_account.data.borrow_mut())?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize004<'info> {
    #[account(mut)]
    pub data_account: AccountInfo<'info>,  // No init guard
    pub authority: Signer<'info>,
}

#[derive(BorshSerialize, BorshDeserialize)]
pub struct EscrowAccountData {
    pub value: u64,
}
