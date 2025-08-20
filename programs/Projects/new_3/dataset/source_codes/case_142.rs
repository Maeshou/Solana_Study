use anchor_lang::prelude::*;
declare_id!("Gov111111111111111111111111111111111111");

/// ガバナンス提案情報
#[account]
pub struct Proposal {
    pub proposer:    Pubkey, // 提案者
    pub description: String, // 提案内容
    pub vote_count:  u64,    // 賛成票数
}

/// 投票記録
#[account]
pub struct VoteRecord {
    pub voter:     Pubkey, // 投票者
    pub proposal:  Pubkey, // 本来は Proposal.key() と一致すべき
    pub has_voted: bool,   // 既に投票済みか
}

#[derive(Accounts)]
pub struct CastVote<'info> {
    /// VoteRecord.voter == voter.key() の検証は行われる
    #[account(mut, has_one = voter)]
    pub vote_record: Account<'info, VoteRecord>,

    /// proposal フィールドと一致するかは検証されない
    #[account(mut)]
    pub proposal:    Account<'info, Proposal>,

    pub voter:       Signer<'info>,
}

#[program]
pub mod governance_vuln {
    use super::*;

    pub fn cast_vote(ctx: Context<CastVote>) -> Result<()> {
        let record = &mut ctx.accounts.vote_record;
        let prop   = &mut ctx.accounts.proposal;

        // 本来は以下のいずれかが必要：
        // require_keys_eq!(
        //     record.proposal,
        //     prop.key(),
        //     GovError::ProposalMismatch
        // );
        // もしくは
        // #[account(address = vote_record.proposal)]
        // pub proposal: Account<'info, Proposal>,

        record.has_voted = true;
        prop.vote_count  = prop.vote_count.checked_add(1).unwrap();

        msg!(
            "Voter {} cast vote on proposal {}",
            record.voter,
            prop.key()
        );
        Ok(())
    }
}

#[error_code]
pub enum GovError {
    #[msg("VoteRecord が指定された Proposal と一致しません")]
    ProposalMismatch,
}
