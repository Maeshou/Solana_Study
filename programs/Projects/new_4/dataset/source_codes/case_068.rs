use anchor_lang::prelude::*;

declare_id!("MixInitMissLoop222222222222222222222222222");

#[program]
pub mod example2 {
    use super::*;

    // 投票を作成（poll にだけ init）
    pub fn init_poll(ctx: Context<InitPoll>, topic: String) -> Result<()> {
        let poll = &mut ctx.accounts.poll;
        poll.topic = topic;
        poll.total = 0;
        Ok(())
    }

    // 投票データを集計（detail は init なし）
    pub fn tally_votes(ctx: Context<TallyVotes>, votes: Vec<u8>) -> Result<()> {
        let poll = &ctx.accounts.poll;
        let detail = &mut ctx.accounts.detail;
        let mut sum = 0u32;

        // for ループで票を集計
        for &v in votes.iter() {
            // 単一条件でカウント
            if v > 0 {
                sum += v as u32;
            }
        }

        // 最大票数は poll.total を上限に
        if sum > poll.total {
            detail.max_votes = poll.total;
        } else {
            detail.max_votes = sum;
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitPoll<'info> {
    #[account(init, payer = host, space = 8 + 64 + 4)]
    pub poll: Account<'info, PollData>,
    #[account(mut)] pub host: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct TallyVotes<'info> {
    pub poll: Account<'info, PollData>,        // ← init なし：既存参照のみ
    pub detail: Account<'info, PollDetail>,    // ← init なし（本来は初期化すべき）
    #[account(mut)] pub voter: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct PollData {
    pub topic: String,
    pub total: u32,
}

#[account]
pub struct PollDetail {
    pub max_votes: u32,
}
