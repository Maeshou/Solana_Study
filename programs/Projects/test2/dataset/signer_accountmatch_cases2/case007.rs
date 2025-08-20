
use anchor_lang::prelude::*;

declare_id!("PollSys77777777777777777777777777777777777");

#[program]
pub mod case7 {
    use super::*;

    pub fn close_poll(ctx: Context<ClosePoll>) -> Result<()> {
        let poll = &mut ctx.accounts.poll;
        poll.status = "closed".to_string();
        poll.last_action = Clock::get()?.unix_timestamp;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct ClosePoll<'info> {
    #[account(mut)]
    pub poll: Account<'info, Poll>,
    /// CHECK: no signer or validation
    pub closer: UncheckedAccount<'info>,
}

#[account]
pub struct Poll {
    pub status: String,
    pub last_action: i64,
    pub owner: Pubkey,
}
