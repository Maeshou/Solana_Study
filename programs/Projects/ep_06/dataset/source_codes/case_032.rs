use anchor_lang::prelude::*;
declare_id!("WRIT0321111111111111111111111111111111111111");

#[program]
pub mod case032 {
    use super::*;
    pub fn execute_writestorage(ctx: Context<WriteStorageContext>) -> Result<()> {
        // Distributed storage logic
        msg!("Accessing storage pool");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct WriteStorageContext<'info> {
    /// CHECK: expecting WriteStorageAccount but using UncheckedAccount
    pub account_a: UncheckedAccount<'info>,
    /// CHECK: expecting WriteStorageAccount but using UncheckedAccount
    pub account_b: UncheckedAccount<'info>,
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct WriteStorageAccount {
    pub dummy: u64,
    pub counter: u64,
    pub version: u8,
}