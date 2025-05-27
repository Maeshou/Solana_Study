
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct SubmitScoreCtxqfoc<'info> {
    #[account(mut)] pub leaderboard: Account<'info, DataAccount>,
    #[account(mut)] pub player: Account<'info, DataAccount>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAccount {
    pub data: u64,
}

#[program]
pub mod missing_owner_check_054 {
    use super::*;

    pub fn submit_score(ctx: Context<SubmitScoreCtxqfoc>, amount: u64) -> Result<()> {
        let acct = &mut ctx.accounts.leaderboard;
        // custom logic for submit_score
        acct.data = acct.data.checked_add(amount).unwrap();
        msg!("Executed submit_score logic");
        Ok(())
    }
}
