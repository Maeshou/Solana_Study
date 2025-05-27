use anchor_lang::prelude::*;
use borsh::{BorshDeserialize, BorshSerialize};

declare_id!("F00000000000000000000000000000046");

#[program]
pub mod case070_proof_of_holding {
    use super::*;

    pub fn initialize_proof_of_holding(ctx: Context<Initialize070>, param: u64) -> Result<()> {
        // Vulnerable initialization without init check
        let mut data = ProofOfHoldingData::try_from_slice(&ctx.accounts.data_account.data.borrow())?;
        data.value = param;  // Overwrites existing data
        data.serialize(&mut *ctx.accounts.data_account.data.borrow_mut())?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize070<'info> {
    #[account(mut)]
    pub data_account: AccountInfo<'info>,  // No init guard
    pub authority: Signer<'info>,
}

#[derive(BorshSerialize, BorshDeserialize)]
pub struct ProofOfHoldingData {
    pub value: u64,
}
