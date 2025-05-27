use anchor_lang::prelude::*;
use borsh::{BorshDeserialize, BorshSerialize};

declare_id!("F00000000000000000000000000000032");

#[program]
pub mod case050_aml_flag {
    use super::*;

    pub fn initialize_aml_flag(ctx: Context<Initialize050>, param: u64) -> Result<()> {
        // Vulnerable initialization without init check
        let mut data = AmlFlagData::try_from_slice(&ctx.accounts.data_account.data.borrow())?;
        data.value = param;  // Overwrites existing data
        data.serialize(&mut *ctx.accounts.data_account.data.borrow_mut())?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize050<'info> {
    #[account(mut)]
    pub data_account: AccountInfo<'info>,  // No init guard
    pub authority: Signer<'info>,
}

#[derive(BorshSerialize, BorshDeserialize)]
pub struct AmlFlagData {
    pub value: u64,
}
