use anchor_lang::prelude::*;

declare_id!("SafeEx05XXXXXXX5555555555555555555555555555");

#[program]
pub mod example5 {
    use super::*;

    pub fn init_poll(
        ctx: Context<InitPoll>,
        votes_a: u32,
        votes_b: u32,
    ) -> Result<()> {
        let poll = &mut ctx.accounts.poll;
        poll.total = votes_a + votes_b;

        let count_a = &mut ctx.accounts.count_a;
        count_a.votes = votes_a;
        // 偶数票ならボーナス
        if votes_a % 2 == 0 {
            count_a.votes += 2;
        }

        let count_b = &mut ctx.accounts.count_b;
        count_b.votes = votes_b;
        if votes_b % 2 == 1 {
            count_b.votes += 1;
        }
        Ok(())
    }

    pub fn cast_vote(
        ctx: Context<CastVote>,
        vote_for_a: bool,
    ) -> Result<()> {
        let poll = &mut ctx.accounts.poll;
        if vote_for_a {
            ctx.accounts.count_a.votes += 1;
        } else {
            ctx.accounts.count_b.votes += 1;
        }
        poll.total += 1;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitPoll<'info> {
    #[account(init, payer = user, space = 8 + 4)]
    pub poll: Account<'info, PollData>,
    #[account(init, payer = user, space = 8 + 4)]
    pub count_a: Account<'info, VoteCount>,
    #[account(init, payer = user, space = 8 + 4)]
    pub count_b: Account<'info, VoteCount>,
    #[account(mut)] pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CastVote<'info> {
    #[account(mut)] pub poll: Account<'info, PollData>,
    #[account(mut)] pub count_a: Account<'info, VoteCount>,
    #[account(mut)] pub count_b: Account<'info, VoteCount>,
}

#[account]
pub struct PollData {
    pub total: u32,
}

#[account]
pub struct VoteCount {
    pub votes: u32,
}
