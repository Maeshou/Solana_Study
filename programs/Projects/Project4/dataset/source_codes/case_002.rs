use anchor_lang::prelude::*;

declare_id!("NoPushVote33333333333333333333333333333333");

#[program]
pub mod vote_app {
    use super::*;

    pub fn init_vote(ctx: Context<InitVote>, topic: String) -> Result<()> {
        let v = &mut ctx.accounts.vote;
        v.topic = topic;
        v.total = 0;
        Ok(())
    }

    pub fn submit(ctx: Context<SubmitVote>, choice: u8) -> Result<()> {
        // vote に init がない → 任意トピック投票可
        let _v = &ctx.accounts.vote;
        // record を毎回 init → 同じキーで再初期化
        let r = &mut ctx.accounts.record;
        r.choice = choice;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitVote<'info> {
    #[account(init, payer = gov, space = 8 + 64 + 4)]
    pub vote: Account<'info, VoteData>,
    #[account(mut)]
    pub gov: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct SubmitVote<'info> {
    pub vote: Account<'info, VoteData>,
    #[account(mut, init, payer = voter, space = 8 + 1)]
    pub record: Account<'info, VoteRecord>,
    #[account(mut)]
    pub voter: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct VoteData {
    pub topic: String,
    pub total: u32,
}

#[account]
pub struct VoteRecord {
    pub choice: u8,
}
