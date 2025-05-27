
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct CastVoteCtxghyi<'info> {
    #[account(mut)] pub proposal: Account<'info, DataAccount>,
    #[account(mut)] pub voter: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAccount {
    pub data: u64,
}

#[program]
pub mod missing_owner_check_033 {
    use super::*;

    pub fn cast_vote(ctx: Context<CastVoteCtxghyi>, amount: u64) -> Result<()> {
        let acct = &mut ctx.accounts.proposal;
        // custom logic for cast_vote
        for _ in 0..amount { acct.data += 1; }
        msg!("Executed cast_vote logic");
        Ok(())
    }
}
