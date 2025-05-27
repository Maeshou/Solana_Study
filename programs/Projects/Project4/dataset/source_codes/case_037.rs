use anchor_lang::prelude::*;
use borsh::{BorshDeserialize, BorshSerialize};

declare_id!("F00000000000000000000000000000025");

#[program]
pub mod case037_dao_fund {
    use super::*;

    pub fn initialize_dao_fund(ctx: Context<Initialize037>, param: u64) -> Result<()> {
        // Vulnerable initialization without init check
        let mut data = DaoFundData::try_from_slice(&ctx.accounts.data_account.data.borrow())?;
        data.value = param;  // Overwrites existing data
        data.serialize(&mut *ctx.accounts.data_account.data.borrow_mut())?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize037<'info> {
    #[account(mut)]
    pub data_account: AccountInfo<'info>,  // No init guard
    pub authority: Signer<'info>,
}

#[derive(BorshSerialize, BorshDeserialize)]
pub struct DaoFundData {
    pub value: u64,
}
