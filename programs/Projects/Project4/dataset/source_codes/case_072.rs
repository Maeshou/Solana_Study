use anchor_lang::prelude::*;
use borsh::{BorshDeserialize, BorshSerialize};

declare_id!("F00000000000000000000000000000048");

#[program]
pub mod case072_bridge_contract {
    use super::*;

    pub fn initialize_bridge_contract(ctx: Context<Initialize072>, param: u64) -> Result<()> {
        // Vulnerable initialization without init check
        let mut data = BridgeContractData::try_from_slice(&ctx.accounts.data_account.data.borrow())?;
        data.value = param;  // Overwrites existing data
        data.serialize(&mut *ctx.accounts.data_account.data.borrow_mut())?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize072<'info> {
    #[account(mut)]
    pub data_account: AccountInfo<'info>,  // No init guard
    pub authority: Signer<'info>,
}

#[derive(BorshSerialize, BorshDeserialize)]
pub struct BridgeContractData {
    pub value: u64,
}
