use anchor_lang::prelude::*;
use borsh::{BorshDeserialize, BorshSerialize};

declare_id!("F00000000000000000000000000000047");

#[program]
pub mod case071_token_swap {
    use super::*;

    pub fn initialize_token_swap(ctx: Context<Initialize071>, param: u64) -> Result<()> {
        // Vulnerable initialization without init check
        let mut data = TokenSwapData::try_from_slice(&ctx.accounts.data_account.data.borrow())?;
        data.value = param;  // Overwrites existing data
        data.serialize(&mut *ctx.accounts.data_account.data.borrow_mut())?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize071<'info> {
    #[account(mut)]
    pub data_account: AccountInfo<'info>,  // No init guard
    pub authority: Signer<'info>,
}

#[derive(BorshSerialize, BorshDeserialize)]
pub struct TokenSwapData {
    pub value: u64,
}
