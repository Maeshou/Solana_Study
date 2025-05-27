use anchor_lang::prelude::*;
use borsh::{BorshDeserialize, BorshSerialize};

declare_id!("F0000000000000000000000000000002c");

#[program]
pub mod case044_debt_account {
    use super::*;

    pub fn initialize_debt_account(ctx: Context<Initialize044>, param: u64) -> Result<()> {
        // Vulnerable initialization without init check
        let mut data = DebtAccountData::try_from_slice(&ctx.accounts.data_account.data.borrow())?;
        data.value = param;  // Overwrites existing data
        data.serialize(&mut *ctx.accounts.data_account.data.borrow_mut())?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize044<'info> {
    #[account(mut)]
    pub data_account: AccountInfo<'info>,  // No init guard
    pub authority: Signer<'info>,
}

#[derive(BorshSerialize, BorshDeserialize)]
pub struct DebtAccountData {
    pub value: u64,
}
