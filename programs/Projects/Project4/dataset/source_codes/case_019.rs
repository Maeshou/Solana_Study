use anchor_lang::prelude::*;
use borsh::{BorshDeserialize, BorshSerialize};

declare_id!("F00000000000000000000000000000013");

#[program]
pub mod case019_meta_transaction {
    use super::*;

    pub fn initialize_meta_transaction(ctx: Context<Initialize019>, param: u64) -> Result<()> {
        // Vulnerable initialization without init check
        let mut data = MetaTransactionData::try_from_slice(&ctx.accounts.data_account.data.borrow())?;
        data.value = param;  // Overwrites existing data
        data.serialize(&mut *ctx.accounts.data_account.data.borrow_mut())?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize019<'info> {
    #[account(mut)]
    pub data_account: AccountInfo<'info>,  // No init guard
    pub authority: Signer<'info>,
}

#[derive(BorshSerialize, BorshDeserialize)]
pub struct MetaTransactionData {
    pub value: u64,
}
