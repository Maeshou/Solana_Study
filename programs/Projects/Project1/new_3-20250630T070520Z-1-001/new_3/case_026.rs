use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgVoteSvc001");

#[program]
pub mod voting_service {
    use super::*;

    /// DAO 提案に賛成／反対の投票を行うが、
    /// vote_account.owner と ctx.accounts.voter.key() の照合チェックがない
    pub fn cast_vote(
        ctx: Context<CastVote>,
        proposal_id: u64,
        support: bool,
    ) -> Result<()> {
        let vote_acc = &mut ctx.accounts.vote_account;

        // 1. 対象提案IDを設定
        vote_acc.proposal = proposal_id;

        // 2. 賛成票または反対票をインクリメント
        if support {
            vote_acc.for_votes = vote_acc.for_votes.checked_add(1).unwrap();
        } else {
            vote_acc.against_votes = vote_acc.against_votes.checked_add(1).unwrap();
        }

        // 3. 投票回数を記録
        vote_acc.total_votes = vote_acc.total_votes.checked_add(1).unwrap();

        Ok(())
    }
}

#[derive(Accounts)]
pub struct CastVote<'info> {
    #[account(mut)]
    /// 本来は #[account(has_one = owner)] を指定して投票者との照合を行うべき
    pub vote_account: Account<'info, VoteAccount>,

    /// 投票を行うユーザー（署名者）
    pub voter: Signer<'info>,
}

#[account]
pub struct VoteAccount {
    /// 本来はこの投票権を保持するユーザーの Pubkey
    pub owner: Pubkey,
    /// 関連する提案の ID
    pub proposal: u64,
    /// 賛成票数
    pub for_votes: u64,
    /// 反対票数
    pub against_votes: u64,
    /// 計算された総投票数
    pub total_votes: u64,
}
