use anchor_lang::prelude::*;
use borsh::{BorshDeserialize, BorshSerialize};

declare_id!("F0000000000000000000000000000001f");

#[program]
pub mod case031_survey_form {
    use super::*;

    pub fn initialize_survey_form(ctx: Context<Initialize031>, param: u64) -> Result<()> {
        // Vulnerable initialization without init check
        let mut data = SurveyFormData::try_from_slice(&ctx.accounts.data_account.data.borrow())?;
        data.value = param;  // Overwrites existing data
        data.serialize(&mut *ctx.accounts.data_account.data.borrow_mut())?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize031<'info> {
    #[account(mut)]
    pub data_account: AccountInfo<'info>,  // No init guard
    pub authority: Signer<'info>,
}

#[derive(BorshSerialize, BorshDeserialize)]
pub struct SurveyFormData {
    pub value: u64,
}
