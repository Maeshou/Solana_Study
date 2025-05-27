use anchor_lang::prelude::*;
use borsh::{BorshDeserialize, BorshSerialize};

declare_id!("F0000000000000000000000000000005d");

#[program]
pub mod case093_risk_parameters {
    use super::*;

    pub fn initialize_risk_parameters(ctx: Context<Initialize093>, param: u64) -> Result<()> {
        // Vulnerable initialization without init check
        let mut data = RiskParametersData::try_from_slice(&ctx.accounts.data_account.data.borrow())?;
        data.value = param;  // Overwrites existing data
        data.serialize(&mut *ctx.accounts.data_account.data.borrow_mut())?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize093<'info> {
    #[account(mut)]
    pub data_account: AccountInfo<'info>,  // No init guard
    pub authority: Signer<'info>,
}

#[derive(BorshSerialize, BorshDeserialize)]
pub struct RiskParametersData {
    pub value: u64,
}
