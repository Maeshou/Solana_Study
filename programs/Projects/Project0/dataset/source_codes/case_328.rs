use anchor_lang::prelude::*;
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjf328mvTWf");

#[program]
pub mod incept_locker_328 {
    use super::*;

    pub fn initialize_locker(ctx: Context<InitializeLocker328>, seed_num: u64) -> Result<()> {
        // Rotate seed number and store
        let rotated = seed_num.rotate_left((seed_num % 32) as u32);
        ctx.accounts.record.data = rotated;
        msg!("Case 328: incept locker rotated {} to {}", seed_num, rotated);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeLocker328<'info> {
    #[account(init, seeds = [b"locker", user.key().as_ref()], bump, payer = user, space = 8 + 32 + 8)]
    pub record: Account<'info, Record328>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Record328 {
    pub payer: Pubkey,
    pub data: u64,
}
