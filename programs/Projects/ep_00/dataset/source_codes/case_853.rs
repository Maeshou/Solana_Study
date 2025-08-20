use anchor_lang::prelude::*;

declare_id!("REVIVAL853XXXXXXXXXXXXXXX");

#[program]
pub mod safe_revival_853 {
    use super::*;

    pub fn close_account(ctx: Context<Close853>) -> ProgramResult {
        // Secure close using Anchor constraint
        // Account will be closed and lamports transferred automatically
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Close853<'info> {
    #[account(mut, close = receiver)]
    pub target: Account<'info, DataAcc>,
    #[account(mut)]
    pub receiver: SystemAccount<'info>,
}

#[account]
pub struct DataAcc {
    pub data: u64,
}