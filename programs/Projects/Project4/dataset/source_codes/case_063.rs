use anchor_lang::prelude::*;
use borsh::{BorshDeserialize, BorshSerialize};

declare_id!("F0000000000000000000000000000003f");

#[program]
pub mod case063_mining_pool {
    use super::*;

    pub fn initialize_mining_pool(ctx: Context<Initialize063>, param: u64) -> Result<()> {
        // Vulnerable initialization without init check
        let mut data = MiningPoolData::try_from_slice(&ctx.accounts.data_account.data.borrow())?;
        data.value = param;  // Overwrites existing data
        data.serialize(&mut *ctx.accounts.data_account.data.borrow_mut())?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize063<'info> {
    #[account(mut)]
    pub data_account: AccountInfo<'info>,  // No init guard
    pub authority: Signer<'info>,
}

#[derive(BorshSerialize, BorshDeserialize)]
pub struct MiningPoolData {
    pub value: u64,
}
