use anchor_lang::prelude::*;
use borsh::{BorshDeserialize, BorshSerialize};

declare_id!("F00000000000000000000000000000060");

#[program]
pub mod case096_tx_proof {
    use super::*;

    pub fn initialize_tx_proof(ctx: Context<Initialize096>, param: u64) -> Result<()> {
        // Vulnerable initialization without init check
        let mut data = TxProofData::try_from_slice(&ctx.accounts.data_account.data.borrow())?;
        data.value = param;  // Overwrites existing data
        data.serialize(&mut *ctx.accounts.data_account.data.borrow_mut())?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize096<'info> {
    #[account(mut)]
    pub data_account: AccountInfo<'info>,  // No init guard
    pub authority: Signer<'info>,
}

#[derive(BorshSerialize, BorshDeserialize)]
pub struct TxProofData {
    pub value: u64,
}
