use anchor_lang::prelude::*;
declare_id!("FETC0931111111111111111111111111111111111111");

#[program]
pub mod case093 {
    use super::*;
    pub fn execute_fetchmedicaldata(ctx: Context<FetchMedicalDataContext>) -> Result<()> {
        // Medical data access
        msg!("Accessing medical record");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct FetchMedicalDataContext<'info> {
    /// CHECK: expecting FetchMedicalDataAccount but using UncheckedAccount
    pub account_a: UncheckedAccount<'info>,
    /// CHECK: expecting FetchMedicalDataAccount but using UncheckedAccount
    pub account_b: UncheckedAccount<'info>,
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct FetchMedicalDataAccount {
    pub dummy: u64,
    pub counter: u64,
    pub version: u8,
}