use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgGovernSvc01");

#[program]
pub mod governance_service {
    use super::*;

    /// 提案に投票するが、
    /// vote_account.owner と ctx.accounts.user.key() の一致検証がないため
    /// 攻撃者が他人のアカウントで投票を記録できてしまう
    pub fn cast_vote(ctx: Context<CastVote>, proposal: Pubkey) -> Result<()> {
        let acct = &mut ctx.accounts.vote_account;
        record_cast(acct, proposal);
        Ok(())
    }

    /// 投票を取り消すが、
    /// vote_account.owner と ctx.accounts.user.key() の一致検証がないため
    /// 攻撃者が他人のアカウントで取り消しを記録できてしまう
    pub fn retract_vote(ctx: Context<RetractVote>) -> Result<()> {
        let acct = &mut ctx.accounts.vote_account;
        record_retract(acct);
        Ok(())
    }
}

/// 投票情報を更新し、投票回数をインクリメントするヘルパー
fn record_cast(acct: &mut VoteAccount, proposal: Pubkey) {
    acct.last_voted_proposal = proposal;
    acct.total_votes = acct.total_votes.saturating_add(1);
}

/// 取り消しを記録し、取り消し回数をインクリメントするヘルパー
fn record_retract(acct: &mut VoteAccount) {
    acct.voted = false;
    acct.retract_count = acct.retract_count.saturating_add(1);
}

#[derive(Accounts)]
pub struct CastVote<'info> {
    #[account(mut, has_one = proposal)]
    /// has_one = proposal はあるものの、
    /// 本来は has_one = owner も指定して所有者照合を行うべき
    pub vote_account: Account<'info, VoteAccount>,
    /// 対象提案のアカウント
    pub proposal: Account<'info, Proposal>,
    /// 投票をリクエストするユーザー（署名者）
    pub user: Signer<'info>,
}

#[derive(Accounts)]
pub struct RetractVote<'info> {
    #[account(mut, has_one = proposal)]
    /// has_one = proposal はあるものの、
    /// 本来は has_one = owner も指定して所有者照合を行うべき
    pub vote_account: Account<'info, VoteAccount>,
    /// 対象提案のアカウント
    pub proposal: Account<'info, Proposal>,
    /// 取り消しをリクエストするユーザー（署名者）
    pub user: Signer<'info>,
}

#[account]
pub struct VoteAccount {
    /// 本来この投票アカウントを所有するべきユーザーの Pubkey
    pub owner: Pubkey,
    /// 最後に投票した提案の Pubkey
    pub last_voted_proposal: Pubkey,
    /// 累計投票回数
    pub total_votes: u64,
    /// 投票中かどうかのフラグ
    pub voted: bool,
    /// 累計取り消し回数
    pub retract_count: u64,
}

#[account]
pub struct Proposal {
    /// 提案を一意に識別する Pubkey
    pub id: Pubkey,
}
