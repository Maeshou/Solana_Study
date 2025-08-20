use anchor_lang::prelude::*;
declare_id!("READ0331111111111111111111111111111111111111");

#[program]
pub mod case033 {
    use super::*;
    pub fn execute_readstorage(ctx: Context<ReadStorageContext>) -> Result<()> {
        // Distributed storage logic
        msg!("Accessing storage pool");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct ReadStorageContext<'info> {
    /// CHECK: expecting ReadStorageAccount but using UncheckedAccount
    pub account_a: UncheckedAccount<'info>,
    /// CHECK: expecting ReadStorageAccount but using UncheckedAccount
    pub account_b: UncheckedAccount<'info>,
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct ReadStorageAccount {
    pub dummy: u64,
    pub counter: u64,
    pub version: u8,
}