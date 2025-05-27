use anchor_lang::prelude::*;
use borsh::{BorshDeserialize, BorshSerialize};

declare_id!("F00000000000000000000000000000019");

#[program]
pub mod case025_flash_loan {
    use super::*;

    pub fn initialize_flash_loan(ctx: Context<Initialize025>, param: u64) -> Result<()> {
        // Vulnerable initialization without init check
        let mut data = FlashLoanData::try_from_slice(&ctx.accounts.data_account.data.borrow())?;
        data.value = param;  // Overwrites existing data
        data.serialize(&mut *ctx.accounts.data_account.data.borrow_mut())?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize025<'info> {
    #[account(mut)]
    pub data_account: AccountInfo<'info>,  // No init guard
    pub authority: Signer<'info>,
}

#[derive(BorshSerialize, BorshDeserialize)]
pub struct FlashLoanData {
    pub value: u64,
}
