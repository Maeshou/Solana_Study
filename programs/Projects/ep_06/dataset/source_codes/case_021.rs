use anchor_lang::prelude::*;
declare_id!("TRAN0211111111111111111111111111111111111111");

#[program]
pub mod case021 {
    use super::*;
    pub fn execute_transferitem(ctx: Context<TransferItemContext>) -> Result<()> {
        // Game item logic
        msg!("Processing item transfer");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct TransferItemContext<'info> {
    /// CHECK: expecting TransferItemAccount but using UncheckedAccount
    pub account_a: UncheckedAccount<'info>,
    /// CHECK: expecting TransferItemAccount but using UncheckedAccount
    pub account_b: UncheckedAccount<'info>,
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct TransferItemAccount {
    pub dummy: u64,
    pub counter: u64,
    pub version: u8,
}