use anchor_lang::prelude::*;
use borsh::{BorshDeserialize, BorshSerialize};

declare_id!("F0000000000000000000000000000001d");

#[program]
pub mod case029_badge_system {
    use super::*;

    pub fn initialize_badge_system(ctx: Context<Initialize029>, param: u64) -> Result<()> {
        // Vulnerable initialization without init check
        let mut data = BadgeSystemData::try_from_slice(&ctx.accounts.data_account.data.borrow())?;
        data.value = param;  // Overwrites existing data
        data.serialize(&mut *ctx.accounts.data_account.data.borrow_mut())?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize029<'info> {
    #[account(mut)]
    pub data_account: AccountInfo<'info>,  // No init guard
    pub authority: Signer<'info>,
}

#[derive(BorshSerialize, BorshDeserialize)]
pub struct BadgeSystemData {
    pub value: u64,
}
