
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct ClaimPrizeCtxmxol<'info> {
    #[account(mut)] pub leaderboard: Account<'info, DataAccount>,
    #[account(mut)] pub player: Account<'info, DataAccount>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAccount {
    pub data: u64,
}

#[program]
pub mod missing_owner_check_055 {
    use super::*;

    pub fn claim_prize(ctx: Context<ClaimPrizeCtxmxol>, amount: u64) -> Result<()> {
        let acct = &mut ctx.accounts.leaderboard;
        // custom logic for claim_prize
        assert!(ctx.accounts.leaderboard.data > 0); acct.data -= amount;
        msg!("Executed claim_prize logic");
        Ok(())
    }
}
