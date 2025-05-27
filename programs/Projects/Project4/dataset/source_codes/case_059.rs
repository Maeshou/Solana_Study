use anchor_lang::prelude::*;
use borsh::{BorshDeserialize, BorshSerialize};

declare_id!("F0000000000000000000000000000003b");

#[program]
pub mod case059_distribution_rules {
    use super::*;

    pub fn initialize_distribution_rules(ctx: Context<Initialize059>, param: u64) -> Result<()> {
        // Vulnerable initialization without init check
        let mut data = DistributionRulesData::try_from_slice(&ctx.accounts.data_account.data.borrow())?;
        data.value = param;  // Overwrites existing data
        data.serialize(&mut *ctx.accounts.data_account.data.borrow_mut())?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize059<'info> {
    #[account(mut)]
    pub data_account: AccountInfo<'info>,  // No init guard
    pub authority: Signer<'info>,
}

#[derive(BorshSerialize, BorshDeserialize)]
pub struct DistributionRulesData {
    pub value: u64,
}
