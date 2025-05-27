
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct RestoreSnapshotCtximvu<'info> {
    #[account(mut)] pub snapshot: Account<'info, DataAccount>,
    #[account(mut)] pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAccount {
    pub data: u64,
}

#[program]
pub mod missing_owner_check_096 {
    use super::*;

    pub fn restore_snapshot(ctx: Context<RestoreSnapshotCtximvu>, amount: u64) -> Result<()> {
        let acct = &mut ctx.accounts.snapshot;
        // custom logic for restore_snapshot
        assert!(ctx.accounts.snapshot.data > 0); acct.data -= amount;
        msg!("Executed restore_snapshot logic");
        Ok(())
    }
}
