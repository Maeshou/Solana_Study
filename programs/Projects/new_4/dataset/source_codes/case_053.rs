// 8. 投票＋投票履歴
use anchor_lang::prelude::*;
declare_id!("VOTE111122223333444455556666777788");

#[program]
pub mod misinit_poll_v6 {
    use super::*;

    pub fn init_poll(
        ctx: Context<InitPoll>,
        question: String,
    ) -> Result<()> {
        let p = &mut ctx.accounts.poll;
        p.question = question;
        p.yes = 0;
        p.no = 0;
        Ok(())
    }

    pub fn vote(
        ctx: Context<InitPoll>,
        agree: bool,
    ) -> Result<()> {
        let p = &mut ctx.accounts.poll;
        if agree { p.yes += 1; } else { p.no += 1; }
        Ok(())
    }

    pub fn record_vote(
        ctx: Context<InitPoll>,
        voter: Pubkey,
    ) -> Result<()> {
        let log = &mut ctx.accounts.vote_log;
        log.voters.push(voter);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitPoll<'info> {
    #[account(init, payer = user, space = 8 + (4+128) + 8)] pub poll: Account<'info, PollData>,
    #[account(mut)] pub vote_log: Account<'info, VoteLog>,
    #[account(mut)] pub user: Signer<'info>, pub system_program: Program<'info, System>,
}

#[account]
pub struct PollData { pub question: String, pub yes:u64, pub no:u64 }
#[account]
pub struct VoteLog { pub voters: Vec<Pubkey> }