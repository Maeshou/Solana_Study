use anchor_lang::prelude::*;
use borsh::{BorshDeserialize, BorshSerialize};

declare_id!("F00000000000000000000000000000002");

#[program]
pub mod case002_token_balance {
    use super::*;

    pub fn initialize_token_balance(ctx: Context<Initialize002>, param: u64) -> Result<()> {
        // Vulnerable initialization without init check
        let mut data = TokenBalanceData::try_from_slice(&ctx.accounts.data_account.data.borrow())?;
        data.value = param;  // Overwrites existing data
        data.serialize(&mut *ctx.accounts.data_account.data.borrow_mut())?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize002<'info> {
    #[account(mut)]
    pub data_account: AccountInfo<'info>,  // No init guard
    pub authority: Signer<'info>,
}

#[derive(BorshSerialize, BorshDeserialize)]
pub struct TokenBalanceData {
    pub value: u64,
}
