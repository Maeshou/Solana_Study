use anchor_lang::prelude::*;
use borsh::{BorshDeserialize, BorshSerialize};

declare_id!("F00000000000000000000000000000049");

#[program]
pub mod case073_cross_chain_status {
    use super::*;

    pub fn initialize_cross_chain_status(ctx: Context<Initialize073>, param: u64) -> Result<()> {
        // Vulnerable initialization without init check
        let mut data = CrossChainStatusData::try_from_slice(&ctx.accounts.data_account.data.borrow())?;
        data.value = param;  // Overwrites existing data
        data.serialize(&mut *ctx.accounts.data_account.data.borrow_mut())?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize073<'info> {
    #[account(mut)]
    pub data_account: AccountInfo<'info>,  // No init guard
    pub authority: Signer<'info>,
}

#[derive(BorshSerialize, BorshDeserialize)]
pub struct CrossChainStatusData {
    pub value: u64,
}
