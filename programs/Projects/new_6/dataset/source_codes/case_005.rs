// ========================================
// 5. 脆弱な投票システム - Vulnerable Voting System
// ========================================

use anchor_lang::prelude::*;

declare_id!("V5uLnErAbLeCoD3F0r3xAmP1e5tUdY7BaTt1eAr3nA4x");

#[program]
pub mod vulnerable_voting {
    use super::*;

    pub fn init_governance(ctx: Context<InitGovernance>) -> Result<()> {
        let governance = &mut ctx.accounts.governance;
        governance.admin = ctx.accounts.admin.key();
        governance.total_proposals = 0;
        governance.active_voters = 0;
        Ok(())
    }

    pub fn create_proposal(ctx: Context<CreateProposal>, description: String) -> Result<()> {
        let proposal = &mut ctx.accounts.proposal;
        proposal.governance = ctx.accounts.governance.key();
        proposal.proposer = ctx.accounts.proposer.key();
        proposal.description = description;
        proposal.yes_votes = 0;
        proposal.no_votes = 0;
        proposal.active = true;

        let governance = &mut ctx.accounts.governance;
        governance.total_proposals = governance.total_proposals.checked_add(1).unwrap_or(u64::MAX);
        Ok(())
    }

    // 脆弱性: invoke_signedとUncheckedAccountの組み合わせ
    pub fn vulnerable_vote(ctx: Context<VulnerableVote>) -> Result<()> {
        let governance = &mut ctx.accounts.governance;
        
        // 脆弱性: UncheckedAccountで投票者検証なし
        let voter_a_info = &ctx.accounts.voter_a;
        let voter_b_info = &ctx.accounts.voter_b;

        // 脆弱性: 同一投票者が複数回投票可能
        let voter_a_balance = **voter_a_info.lamports.borrow();
        let voter_b_balance = **voter_b_info.lamports.borrow();

        // 投票処理ループ
        for vote_round in 0..4 {
            if voter_a_balance > voter_b_balance {
                let vote_weight = (voter_a_balance >> (vote_round * 8)) & 0xFF;
                governance.active_voters = governance.active_voters.checked_add(vote_weight as u32).unwrap_or(u32::MAX);
                
                // 投票力の累積計算
                let power_bonus = (vote_weight as u64) * (vote_round + 1) as u64;
                governance.total_proposals = governance.total_proposals.checked_add(power_bonus).unwrap_or(u64::MAX);
                
                msg!("Voter A vote round {}: weight={}", vote_round, vote_weight);
            } else {
                let adjusted_balance = voter_b_balance / ((vote_round + 1) as u64);
                governance.active_voters = governance.active_voters.checked_add(adjusted_balance as u32).unwrap_or(u32::MAX);
                
                // ビット操作による投票調整
                let bit_pattern = (vote_round as u64) << 16;
                let combined_weight = adjusted_balance ^ bit_pattern;
                governance.total_proposals = governance.total_proposals.checked_add(combined_weight).unwrap_or(u64::MAX);
                
                msg!("Voter B vote round {}: adjusted={}", vote_round, adjusted_balance);
            }
        }

        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitGovernance<'info> {
    #[account(init, payer = admin, space = 8 + 32 + 8 + 4)]
    pub governance: Account<'info, Governance>,
    #[account(mut)]
    pub admin: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CreateProposal<'info> {
    #[account(mut)]
    pub governance: Account<'info, Governance>,
    #[account(init, payer = proposer, space = 8 + 32 + 32 + 200 + 8 + 8 + 1)]
    pub proposal: Account<'info, Proposal>,
    #[account(mut)]
    pub proposer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

// 脆弱性: UncheckedAccountで投票者検証なし
#[derive(Accounts)]
pub struct VulnerableVote<'info> {
    #[account(mut)]
    pub governance: Account<'info, Governance>,
    /// CHECK: 脆弱性 - 投票者Aの検証なし
    pub voter_a: UncheckedAccount<'info>,
    /// CHECK: 脆弱性 - 投票者Bの検証なし、同一可能
    pub voter_b: UncheckedAccount<'info>,
    pub authority: Signer<'info>,
}

#[account]
pub struct Governance {
    pub admin: Pubkey,
    pub total_proposals: u64,
    pub active_voters: u32,
}

#[account]
pub struct Proposal {
    pub governance: Pubkey,
    pub proposer: Pubkey,
    pub description: String,
    pub yes_votes: u64,
    pub no_votes: u64,
    pub active: bool,
}
