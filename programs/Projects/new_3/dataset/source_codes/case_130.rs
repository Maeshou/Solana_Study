use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgGovRematch04");

#[program]
pub mod governance_service {
    use super::*;

    /// 提案に賛否を記録するが、
    /// has_one = proposal は検証するものの、
    /// 実際の投票アカウント所有者（owner）照合がないため、
    /// 攻撃者が他人の VoteAccount を指定して投票を記録できてしまう
    pub fn cast_vote(ctx: Context<CastVote>, support: bool) -> Result<()> {
        let acct = &mut ctx.accounts.vote_account;
        // 賛否を設定
        acct.last_support = support;
        // 投票済みフラグを立てる
        acct.voted = true;
        // 投票回数を plain + で更新
        acct.vote_count = acct.vote_count + 1;
        Ok(())
    }

    /// 投票を取り消すが、
    /// has_one = proposal は検証するものの、
    /// 実際の投票アカウント所有者（owner）照合がないため、
    /// 攻撃者が他人の VoteAccount を指定して取り消しを記録できてしまう
    pub fn retract_vote(ctx: Context<RetractVote>) -> Result<()> {
        let acct = &mut ctx.accounts.vote_account;
        // 投票済みフラグを下ろす
        acct.voted = false;
        // 取り消し回数を +1 で更新
        acct.retract_count = acct.retract_count + 1;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CastVote<'info> {
    #[account(mut, has_one = proposal)]
    /// 本来は has_one = owner を追加して所有者照合を行うべき
    pub vote_account: Account<'info, VoteAccount>,

    /// 対象提案アカウント（検証済み）
    pub proposal: Account<'info, Proposal>,

    /// 投票をリクエストするユーザー（署名者・照合漏れ）
    pub user: Signer<'info>,
}

#[derive(Accounts)]
pub struct RetractVote<'info> {
    #[account(mut, has_one = proposal)]
    /// 本来は has_one = owner を追加して所有者照合を行うべき
    pub vote_account: Account<'info, VoteAccount>,

    /// 対象提案アカウント（検証済み）
    pub proposal: Account<'info, Proposal>,

    /// 取り消しをリクエストするユーザー（署名者・照合漏れ）
    pub user: Signer<'info>,
}

#[account]
pub struct VoteAccount {
    /// 本来この投票アカウントを所有するべきユーザーの Pubkey
    pub owner: Pubkey,

    /// 紐づく提案アカウントの Pubkey
    pub proposal: Pubkey,

    /// 投票済みフラグ
    pub voted: bool,

    /// 最後に投じた賛否
    pub last_support: bool,

    /// 累計投票回数
    pub vote_count: u64,

    /// 累計取り消し回数
    pub retract_count: u64,
}

#[account]
pub struct Proposal {
    /// 提案を一意に識別する Pubkey
    pub id: Pubkey,
}
