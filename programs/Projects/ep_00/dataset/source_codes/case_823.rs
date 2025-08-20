use anchor_lang::prelude::*;

declare_id!("REVIVAL823XXXXXXXXXXXXXXX");

#[program]
pub mod safe_revival_823 {
    use super::*;

    pub fn close_account(ctx: Context<Close823>) -> ProgramResult {
        // Secure close using Anchor constraint
        // Account will be closed and lamports transferred automatically
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Close823<'info> {
    #[account(mut, close = receiver)]
    pub target: Account<'info, DataAcc>,
    #[account(mut)]
    pub receiver: SystemAccount<'info>,
}

#[account]
pub struct DataAcc {
    pub data: u64,
}