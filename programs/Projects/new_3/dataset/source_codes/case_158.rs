use anchor_lang::prelude::*;
declare_id!("GovVoteHasOne1111111111111111111111111111");

/// ガバナンス提案
#[account]
pub struct Proposal {
    pub proposer:     Pubkey, // 提案者
    pub approve_count: u64,   // 賛成票数
}

/// 投票記録
#[account]
pub struct VoteRecord {
    pub voter:     Pubkey, // 投票者
    pub proposal:  Pubkey, // 本来は Proposal.key() と一致すべき
    pub has_voted: bool,   // 既に投票済みか
}

#[derive(Accounts)]
pub struct CreateProposal<'info> {
    #[account(init, payer = proposer, space = 8 + 32 + 8)]
    pub proposal:    Account<'info, Proposal>,
    #[account(mut)]
    pub proposer:    Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CastVote<'info> {
    /// VoteRecord.voter == voter.key() は検証される
    #[account(mut, has_one = voter)]
    pub vote_record: Account<'info, VoteRecord>,

    /// Proposal.key() と vote_record.proposal の一致検証がない
    #[account(mut)]
    pub proposal:    Account<'info, Proposal>,

    pub voter:       Signer<'info>,
}

#[program]
pub mod governance_vuln_has_one {
    use super::*;

    /// 提案を登録
    pub fn create_proposal(ctx: Context<CreateProposal>) -> Result<()> {
        let p = &mut ctx.accounts.proposal;
        p.proposer      = ctx.accounts.proposer.key();
        p.approve_count = 0;
        Ok(())
    }

    /// 賛成投票を行う
    pub fn cast_vote(ctx: Context<CastVote>) -> Result<()> {
        let p = &mut ctx.accounts.proposal;
        let r = &mut ctx.accounts.vote_record;

        // ── 脆弱性ポイント ──
        // VoteRecord.proposal と Proposal.key() の整合性検証がないため、
        // 攻撃者は自分で用意した別の VoteRecord を渡して、
        // 任意の Proposal に賛成票を追加できる。

        r.has_voted = true;
        p.approve_count = p.approve_count.checked_add(1).unwrap();
        Ok(())
    }
}
