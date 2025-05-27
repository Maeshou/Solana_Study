use anchor_lang::prelude::*;
use borsh::{BorshDeserialize, BorshSerialize};

declare_id!("F00000000000000000000000000000014");

#[program]
pub mod case020_subscription_plan {
    use super::*;

    pub fn initialize_subscription_plan(ctx: Context<Initialize020>, param: u64) -> Result<()> {
        // Vulnerable initialization without init check
        let mut data = SubscriptionPlanData::try_from_slice(&ctx.accounts.data_account.data.borrow())?;
        data.value = param;  // Overwrites existing data
        data.serialize(&mut *ctx.accounts.data_account.data.borrow_mut())?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize020<'info> {
    #[account(mut)]
    pub data_account: AccountInfo<'info>,  // No init guard
    pub authority: Signer<'info>,
}

#[derive(BorshSerialize, BorshDeserialize)]
pub struct SubscriptionPlanData {
    pub value: u64,
}
