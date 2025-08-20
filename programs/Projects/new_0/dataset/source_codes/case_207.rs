use anchor_lang::prelude::*;
declare_id!("GovSafe1111111111111111111111111111111111");

/// 提案情報
#[account]
pub struct Proposal {
    pub creator:    Pubkey,  // 提案者
    pub title:      String,  // 提案タイトル
    pub vote_count: u64,     // 総賛成票数
}

/// 投票記録
#[account]
pub struct VoteRecord {
    pub voter:       Pubkey,  // 投票したユーザー
    pub proposal:    Pubkey,  // Proposal.key()
    pub support:     bool,    // 賛否フラグ (true = 賛成)
}

#[derive(Accounts)]
pub struct CreateProposal<'info> {
    #[account(init, payer = creator, space = 8 + 32 + 4 + 128 + 8)]
    pub proposal:     Account<'info, Proposal>,
    #[account(mut)]
    pub creator:      Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CastVote<'info> {
    /// Proposal.creator == creator.key() は不要ですが例示
    #[account(mut, has_one = creator)]
    pub proposal:     Account<'info, Proposal>,

    /// VoteRecord.proposal == proposal.key(), VoteRecord.voter == voter.key() を検証
    #[account(
        init,
        payer = voter,
        space = 8 + 32 + 32 + 1,
        has_one = proposal,
        has_one = voter
    )]
    pub vote_record:  Account<'info, VoteRecord>,

    #[account(mut)]
    pub creator:      Signer<'info>,
    #[account(mut)]
    pub voter:        Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct FinalizeVote<'info> {
    /// VoteRecord.proposal == proposal.key()、VoteRecord.voter == caller.key() を検証
    #[account(mut, has_one = proposal, has_one = voter)]
    pub vote_record:  Account<'info, VoteRecord>,

    #[account(mut)]
    pub proposal:     Account<'info, Proposal>,
    #[account(mut)]
    pub voter:        Signer<'info>,
}

#[program]
pub mod governance_safe {
    use super::*;

    /// 提案を作成
    pub fn create_proposal(
        ctx: Context<CreateProposal>,
        title: String
    ) -> Result<()> {
        let p = &mut ctx.accounts.proposal;
        p.creator    = ctx.accounts.creator.key();
        p.title      = title;
        p.vote_count = 0;
        Ok(())
    }

    /// 投票を行う
    pub fn cast_vote(
        ctx: Context<CastVote>,
        support: bool
    ) -> Result<()> {
        let p  = &mut ctx.accounts.proposal;
        let vr = &mut ctx.accounts.vote_record;

        // 明示的にセット
        vr.voter    = ctx.accounts.voter.key();
        vr.proposal = ctx.accounts.proposal.key();
        vr.support  = support;

        // 再チェック（二重保証）
        require_keys_eq!(vr.proposal, p.key(), GovError::ProposalMismatch);
        require_keys_eq!(vr.voter, ctx.accounts.voter.key(), GovError::VoterMismatch);

        // 賛成票ならカウント加算
        require!(support, GovError::NotSupported);
        p.vote_count = p.vote_count
            .checked_add(1)
            .ok_or(GovError::Overflow)?;
        Ok(())
    }

    /// 投票を最終集計
    pub fn finalize_vote(
        ctx: Context<FinalizeVote>
    ) -> Result<()> {
        let p  = &mut ctx.accounts.proposal;
        let vr = &ctx.accounts.vote_record;

        // 再チェック
        require_keys_eq!(vr.proposal, p.key(), GovError::ProposalMismatch);
        require_keys_eq!(vr.voter, ctx.accounts.voter.key(), GovError::VoterMismatch);

        // 実装例として、票数は既に加算済みなので何もしない
        Ok(())
    }
}

#[error_code]
pub enum GovError {
    #[msg("VoteRecord.proposal が Proposal に一致しません")]
    ProposalMismatch,
    #[msg("VoteRecord.voter が署名者に一致しません")]
    VoterMismatch,
    #[msg("賛成 (support=true) のみ許可されています")]
    NotSupported,
    #[msg("票数の加算でオーバーフローが発生しました")]
    Overflow,
}
