use anchor_lang::prelude::*;
use solana_program::program_error::ProgramError;

// Program ID - replace with your own
declare_id!("Fg6PaFpoGXkYsidMpGOVERNaNcE123456789ABCDE");

#[program]
pub mod governance_voting {
    use super::*;

    /// 提案を作成
    pub fn create_proposal(
        ctx: Context<CreateProposal>,
        bump: u8,
        id: u64,
        description: String,
    ) -> ProgramResult {
        // 説明文長チェック
        if description.len() > 200 {
            return Err(ProgramError::InvalidInstructionData.into());
        }
        let prop = &mut ctx.accounts.proposal;
        prop.creator = *ctx.accounts.creator.key;
        prop.id = id;
        prop.description = description;
        prop.votes_for = 0;
        prop.votes_against = 0;
        prop.open = true;
        prop.bump = bump;
        Ok(())
    }

    /// 投票を実行
    pub fn cast_vote(
        ctx: Context<CastVote>,
        support: bool,
    ) -> ProgramResult {
        let prop = &mut ctx.accounts.proposal;
        // 提案オープン状態チェック
        if !prop.open {
            return Err(ProgramError::InvalidAccountData.into());
        }
        // 資料カウント更新
        prop.votes_for = if support {
            prop.votes_for.checked_add(1).ok_or(ProgramError::InvalidInstructionData)?
        } else {
            prop.votes_for
        };
        prop.votes_against = if !support {
            prop.votes_against.checked_add(1).ok_or(ProgramError::InvalidInstructionData)?
        } else {
            prop.votes_against
        };
        Ok(())
    }

    /// 提案を閉鎖
    pub fn close_proposal(
        ctx: Context<CloseProposal>,
    ) -> ProgramResult {
        let prop = &mut ctx.accounts.proposal;
        // 作成者チェック
        if prop.creator != *ctx.accounts.creator.key {
            return Err(ProgramError::InvalidAccountData.into());
        }
        // 状態チェック
        if !prop.open {
            return Err(ProgramError::InvalidAccountData.into());
        }
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
    pub voter: Signer<'info>,
}

#[derive(Accounts)]
pub struct CloseProposal<'info> {
    #[account(
        mut,
        seeds = [b"proposal", &proposal.id.to_le_bytes()],
        bump = proposal.bump,
    )]
    pub proposal: Account<'info, Proposal>,
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
