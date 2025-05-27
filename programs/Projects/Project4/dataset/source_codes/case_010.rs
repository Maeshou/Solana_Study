use anchor_lang::prelude::*;
use borsh::{BorshDeserialize, BorshSerialize};

declare_id!("F0000000000000000000000000000000a");

#[program]
pub mod case010_multisig_wallet {
    use super::*;

    pub fn initialize_multisig_wallet(ctx: Context<Initialize010>, param: u64) -> Result<()> {
        // Vulnerable initialization without init check
        let mut data = MultisigWalletData::try_from_slice(&ctx.accounts.data_account.data.borrow())?;
        data.value = param;  // Overwrites existing data
        data.serialize(&mut *ctx.accounts.data_account.data.borrow_mut())?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize010<'info> {
    #[account(mut)]
    pub data_account: AccountInfo<'info>,  // No init guard
    pub authority: Signer<'info>,
}

#[derive(BorshSerialize, BorshDeserialize)]
pub struct MultisigWalletData {
    pub value: u64,
}
