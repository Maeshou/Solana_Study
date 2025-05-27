use anchor_lang::prelude::*;
use borsh::{BorshDeserialize, BorshSerialize};

declare_id!("F00000000000000000000000000000059");

#[program]
pub mod case089_audit_log {
    use super::*;

    pub fn initialize_audit_log(ctx: Context<Initialize089>, param: u64) -> Result<()> {
        // Vulnerable initialization without init check
        let mut data = AuditLogData::try_from_slice(&ctx.accounts.data_account.data.borrow())?;
        data.value = param;  // Overwrites existing data
        data.serialize(&mut *ctx.accounts.data_account.data.borrow_mut())?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize089<'info> {
    #[account(mut)]
    pub data_account: AccountInfo<'info>,  // No init guard
    pub authority: Signer<'info>,
}

#[derive(BorshSerialize, BorshDeserialize)]
pub struct AuditLogData {
    pub value: u64,
}
