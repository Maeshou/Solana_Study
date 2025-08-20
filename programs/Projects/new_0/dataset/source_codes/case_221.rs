use anchor_lang::prelude::*;

// Program ID - replace with your own
declare_id!("Fg6PaFpoGXkYsidMpA1B2C3D4E5F6G7H8I9J0K1L2M3N4");

#[program]
pub mod governance {
    use super::*;

    /// 提案アカウントを作成
    pub fn create_proposal(
        ctx: Context<CreateProposal>,
        bump: u8,
        id: u64,
        description: String,
    ) -> ProgramResult {
        require!(description.len() <= 200, ErrorCode::DescriptionTooLong);

        let proposal = &mut ctx.accounts.proposal;
        proposal.creator = *ctx.accounts.creator.key;
        proposal.bump = bump;
        proposal.id = id;
        proposal.description = description;
        proposal.votes = 0;
        proposal.open = true;
        Ok(())
    }

    /// 投票を行う
    pub fn cast_vote(
        ctx: Context<CastVote>,
    ) -> ProgramResult {
        let proposal = &mut ctx.accounts.proposal;
        require!(proposal.open, ErrorCode::ProposalClosed);

        // 一度のトランザクションで1票のみ
        proposal.votes = proposal.votes.checked_add(1).ok_or(ErrorCode::Overflow)?;
        Ok(())
    }

    /// 提案を終了する
    pub fn close_proposal(
        ctx: Context<CloseProposal>,
    ) -> ProgramResult {
        let proposal = &mut ctx.accounts.proposal;
        require!(proposal.open, ErrorCode::ProposalClosed);

        proposal.open = false;
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
        space = 8 + 32 + 1 + 8 + 4 + 200 + 8 + 1,
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
    /// CHECK: 投票者の署名確認のみ
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
    /// 提案作成者のみ終了可能
    pub creator: Signer<'info>,
}

#[account]
pub struct Proposal {
    pub creator: Pubkey,
    pub bump: u8,
    pub id: u64,
    pub description: String,
    pub votes: u64,
    pub open: bool,
}

#[error]
pub enum ErrorCode {
    #[msg("Description exceeds maximum length of 200 bytes.")]
    DescriptionTooLong,
    #[msg("Proposal is already closed.")]
    ProposalClosed,
    #[msg("Numeric overflow occurred.")]
    Overflow,
}
