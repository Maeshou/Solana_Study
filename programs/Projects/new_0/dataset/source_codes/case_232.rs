use anchor_lang::prelude::*;

// Program ID - replace with your own
declare_id!("Fg6PaFpoGXkYsidMpGOVERNaNcE123456789ABCDE");

#[program]
pub mod governance_voting {
    use super::*;

    /// 提案を作成: 作成者と説明を登録（最大200文字）
    pub fn create_proposal(
        ctx: Context<CreateProposal>,
        bump: u8,
        id: u64,
        description: String,
    ) -> ProgramResult {
        require!(description.len() <= 200, GovernanceError::DescriptionTooLong);
        let prop = &mut ctx.accounts.proposal;
        prop.creator = *ctx.accounts.creator.key;
        prop.id = id;
        prop.description = description;
        prop.votes_for = 0;
        prop.votes_against = 0;
        prop.open = true;
        prop.bump = bump; // bump フィールドを正しく設定
        Ok(())
    }

    /// 投票を行う: 賛成(true)/反対(false) 1票ずつ
    pub fn cast_vote(
        ctx: Context<CastVote>,
        support: bool,
    ) -> ProgramResult {
        let prop = &mut ctx.accounts.proposal;
        require!(prop.open, GovernanceError::ProposalClosed);
        if support {
            prop.votes_for = prop.votes_for.checked_add(1).ok_or(GovernanceError::Overflow)?;
        } else {
            prop.votes_against = prop.votes_against.checked_add(1).ok_or(GovernanceError::Overflow)?;
        }
        Ok(())
    }

    /// 提案を終了する: 作成者のみ実行可能
    pub fn close_proposal(
        ctx: Context<CloseProposal>,
    ) -> ProgramResult {
        let prop = &mut ctx.accounts.proposal;
        require!(prop.creator == *ctx.accounts.creator.key, GovernanceError::Unauthorized);
        require!(prop.open, GovernanceError::ProposalClosed);
        prop.open = false;
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(bump: u8, id: u64, description: String)]
pub struct CreateProposal<'info> {
    #[account(
        init,
        seeds = [b"proposal", &id.to_le_bytes()],
        bump = bump,
        payer = creator,
        space = 8 + 32 + 1 + 8 + 4 + 200 + 8 + 8 + 1,
    )]
    pub proposal: Account<'info, Proposal>,
    #[account(mut)]
    pub creator: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct CastVote<'info> {
    #[account(
        mut,
        seeds = [b"proposal", &proposal.id.to_le_bytes()],
        bump = proposal.bump,
    )]
    pub proposal: Account<'info, Proposal>,
    /// 投票者の署名確認のみ
    pub voter: Signer<'info>,
}

#[derive(Accounts)]
pub struct CloseProposal<'info> {
    #[account(
        mut,
        seeds = [b"proposal", &proposal.id.to_le_bytes()],
        bump = proposal.bump,
        has_one = creator,
    )]
    pub proposal: Account<'info, Proposal>,
    /// 作成者のみ閉鎖可能
    pub creator: Signer<'info>,
}

#[account]
pub struct Proposal {
    pub creator: Pubkey,
    pub bump: u8,
    pub id: u64,
    pub description: String,
    pub votes_for: u64,
    pub votes_against: u64,
    pub open: bool,
}

#[error]
pub enum GovernanceError {
    #[msg("Description exceeds 200 characters.")]
    DescriptionTooLong,
    #[msg("Proposal is already closed.")]
    ProposalClosed,
    #[msg("You are not authorized to close this proposal.")]
    Unauthorized,
    #[msg("Arithmetic overflow occurred.")]
    Overflow,
}
