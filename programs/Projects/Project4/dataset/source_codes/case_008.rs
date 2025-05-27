use anchor_lang::prelude::*;
use borsh::{BorshDeserialize, BorshSerialize};

declare_id!("F00000000000000000000000000000008");

#[program]
pub mod case008_governance_proposal {
    use super::*;

    pub fn initialize_governance_proposal(ctx: Context<Initialize008>, param: u64) -> Result<()> {
        // Vulnerable initialization without init check
        let mut data = GovernanceProposalData::try_from_slice(&ctx.accounts.data_account.data.borrow())?;
        data.value = param;  // Overwrites existing data
        data.serialize(&mut *ctx.accounts.data_account.data.borrow_mut())?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize008<'info> {
    #[account(mut)]
    pub data_account: AccountInfo<'info>,  // No init guard
    pub authority: Signer<'info>,
}

#[derive(BorshSerialize, BorshDeserialize)]
pub struct GovernanceProposalData {
    pub value: u64,
}
