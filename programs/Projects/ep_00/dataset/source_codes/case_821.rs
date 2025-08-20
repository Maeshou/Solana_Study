use anchor_lang::prelude::*;

declare_id!("REVIVAL821XXXXXXXXXXXXXXX");

#[program]
pub mod safe_revival_821 {
    use super::*;

    pub fn close_account(ctx: Context<Close821>) -> ProgramResult {
        // Secure close using Anchor constraint
        // Account will be closed and lamports transferred automatically
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Close821<'info> {
    #[account(mut, close = receiver)]
    pub target: Account<'info, DataAcc>,
    #[account(mut)]
    pub receiver: SystemAccount<'info>,
}

#[account]
pub struct DataAcc {
    pub data: u64,
}