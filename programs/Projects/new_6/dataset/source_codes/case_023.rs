// ==================== 2. 脆弱なメンバーシップDAO ====================
// 提案者と投票者の検証が甘く、自己投票による不正議決が可能

use anchor_lang::prelude::*;

declare_id!("V2M3E4M5B6E7R8S9H0I1P2D3A4O5G6O7V8E9R0N1");

#[program]
pub mod vulnerable_membership_dao {
    use super::*;
    
    pub fn init_dao_council(
        ctx: Context<InitDaoCouncil>,
        dao_name: String,
        quorum_threshold: u16,
    ) -> Result<()> {
        let council = &mut ctx.accounts.council;
        council.admin = ctx.accounts.admin.key();
        council.dao_name = dao_name;
        council.quorum_threshold = quorum_threshold;
        council.active_proposals = 0;
        council.is_operational = true;
        
        msg!("DAO Council established: {}", council.dao_name);
        Ok(())
    }
    
    pub fn init_membership_record(
        ctx: Context<InitMembershipRecord>,
        member_tier: MemberTier,
        voting_weight: u32,
    ) -> Result<()> {
        let record = &mut ctx.accounts.record;
        record.council = ctx.accounts.council.key();
        record.member = ctx.accounts.member.key();
        record.member_tier = member_tier;
        record.voting_weight = voting_weight;
        record.proposals_created = 0;
        record.is_active = true;
        
        msg!("Member record created with tier: {:?}", member_tier);
        Ok(())
    }
    
    pub fn process_governance_vote(
        ctx: Context<ProcessGovernanceVote>,
        vote_cycles: u16,
        influence_modifier: u8,
    ) -> Result<()> {
        let council = &mut ctx.accounts.council;
        
        // 脆弱性: proposer_data と voter_data が同じアカウントでも検証されない
        let mut cycle = 0u16;
        loop {
            if cycle >= vote_cycles { break; }
            
            if council.is_operational {
                // 運営中DAOでの投票処理
                council.active_proposals += 1;
                let weight_bonus = (cycle % 10) as u32 * influence_modifier as u32;
                
                // シンプルなシフト演算
                let shifted_weight = weight_bonus << 2;
                let final_weight = shifted_weight >> 1;
                
                msg!("Vote cycle {}: weight {}", cycle, final_weight);
            } else {
                // 停止中DAOでの清算処理
                if council.active_proposals > 0 {
                    council.active_proposals -= 1;
                }
                
                // 閾値の動的調整
                council.quorum_threshold = council.quorum_threshold.saturating_sub(5);
                
                msg!("Suspended DAO cycle {}", cycle);
            }
            cycle += 1;
        }
        
        // 最終コンセンサス調整
        let mut consensus = 0u8;
        while consensus < 4 {
            let adjustment = consensus * influence_modifier;
            council.quorum_threshold += adjustment as u16;
            consensus += 1;
        }
        
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitDaoCouncil<'info> {
    #[account(
        init,
        payer = admin,
        space = 8 + 32 + 64 + 2 + 4 + 1
    )]
    pub council: Account<'info, DaoCouncil>,
    #[account(mut)]
    pub admin: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct InitMembershipRecord<'info> {
    pub council: Account<'info, DaoCouncil>,
    #[account(
        init,
        payer = member,
        space = 8 + 32 + 32 + 1 + 4 + 4 + 1
    )]
    pub record: Account<'info, MembershipRecord>,
    #[account(mut)]
    pub member: Signer<'info>,
    pub system_program: Program<'info, System>,
}

// 脆弱性: 提案者と投票者が同じアカウントでも検証されない
#[derive(Accounts)]
pub struct ProcessGovernanceVote<'info> {
    #[account(mut)]
    pub council: Account<'info, DaoCouncil>,
    /// CHECK: 提案者データの検証が不十分
    pub proposer_data: AccountInfo<'info>,
    /// CHECK: 投票者データの検証が不十分
    pub voter_data: AccountInfo<'info>,
    pub governance_executor: Signer<'info>,
}

#[account]
pub struct DaoCouncil {
    pub admin: Pubkey,
    pub dao_name: String,
    pub quorum_threshold: u16,
    pub active_proposals: u32,
    pub is_operational: bool,
}

#[account]
pub struct MembershipRecord {
    pub council: Pubkey,
    pub member: Pubkey,
    pub member_tier: MemberTier,
    pub voting_weight: u32,
    pub proposals_created: u32,
    pub is_active: bool,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq)]
pub enum MemberTier {
    Bronze,
    Silver,
    Gold,
    Platinum,
}

use MemberTier::*;

#[error_code]
pub enum GovernanceError {
    #[msg("Insufficient voting weight")]
    InsufficientWeight,
    #[msg("DAO not operational")]
    DaoNotOperational,
}