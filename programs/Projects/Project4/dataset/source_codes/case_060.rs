use anchor_lang::prelude::*;
use borsh::{BorshDeserialize, BorshSerialize};

declare_id!("F0000000000000000000000000000003c");

#[program]
pub mod case060_variable_reward {
    use super::*;

    pub fn initialize_variable_reward(ctx: Context<Initialize060>, param: u64) -> Result<()> {
        // Vulnerable initialization without init check
        let mut data = VariableRewardData::try_from_slice(&ctx.accounts.data_account.data.borrow())?;
        data.value = param;  // Overwrites existing data
        data.serialize(&mut *ctx.accounts.data_account.data.borrow_mut())?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize060<'info> {
    #[account(mut)]
    pub data_account: AccountInfo<'info>,  // No init guard
    pub authority: Signer<'info>,
}

#[derive(BorshSerialize, BorshDeserialize)]
pub struct VariableRewardData {
    pub value: u64,
}
