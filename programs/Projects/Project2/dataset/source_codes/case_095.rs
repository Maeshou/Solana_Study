
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct TakeSnapshotCtxpshs<'info> {
    #[account(mut)] pub snapshot: Account<'info, DataAccount>,
    #[account(mut)] pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAccount {
    pub data: u64,
}

#[program]
pub mod missing_owner_check_095 {
    use super::*;

    pub fn take_snapshot(ctx: Context<TakeSnapshotCtxpshs>, amount: u64) -> Result<()> {
        let acct = &mut ctx.accounts.snapshot;
        // custom logic for take_snapshot
        let temp = acct.data; acct.data = temp.checked_mul(2).unwrap();
        msg!("Executed take_snapshot logic");
        Ok(())
    }
}
