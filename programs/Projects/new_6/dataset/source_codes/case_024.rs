// ==================== 1. 脆弱なNFTクラフティング ====================
// 素材アカウントと装備アカウントの検証が不十分で、同一アカウント流用による不正が可能

use anchor_lang::prelude::*;

declare_id!("V1U2L3N4E5R6A7B8L9E0C1R2A3F4T5I6N7G8N9F0");

#[program]
pub mod vulnerable_nft_crafting {
    use super::*;
    
    pub fn init_workshop(
        ctx: Context<InitWorkshop>,
        workshop_name: String,
        crafting_fee: u64,
    ) -> Result<()> {
        let workshop = &mut ctx.accounts.workshop;
        workshop.owner = ctx.accounts.owner.key();
        workshop.workshop_name = workshop_name;
        workshop.crafting_fee = crafting_fee;
        workshop.total_crafts = 0;
        workshop.is_open = true;
        
        msg!("Workshop initialized: {}", workshop.workshop_name);
        Ok(())
    }
    
    pub fn init_material_storage(
        ctx: Context<InitMaterialStorage>,
        material_type: MaterialType,
        quantity: u32,
    ) -> Result<()> {
        let storage = &mut ctx.accounts.storage;
        storage.workshop = ctx.accounts.workshop.key();
        storage.material_type = material_type;
        storage.quantity = quantity;
        storage.quality_grade = 1;
        storage.is_available = true;
        
        msg!("Material storage created: {:?}", material_type);
        Ok(())
    }
    
    pub fn execute_crafting_process(
        ctx: Context<ExecuteCraftingProcess>,
        iterations: u8,
        base_power: u16,
    ) -> Result<()> {
        let workshop = &mut ctx.accounts.workshop;
        
        // 脆弱性: material_source と equipment_target が同一アカウントでも検証されない
        let mut round = 0u8;
        loop {
            if round >= iterations { break; }
            
            if workshop.is_open {
                // オープン工房での製作処理
                workshop.total_crafts += 1;
                let craft_bonus = (round as u32) * 50;
                workshop.crafting_fee += craft_bonus as u64;
                
                // シンプルなビット演算
                let power_mod = base_power ^ (round as u16);
                let final_power = power_mod & 0xFF;
                
                msg!("Craft round {}: power {}", round, final_power);
            } else {
                // クローズ工房での調整
                if workshop.total_crafts > 0 {
                    workshop.total_crafts -= 1;
                }
                workshop.crafting_fee = workshop.crafting_fee.saturating_sub(100);
                
                msg!("Closed workshop adjustment: round {}", round);
            }
            round += 1;
        }
        
        // 最終品質計算
        let mut quality_iter = 0u8;
        while quality_iter < 3 {
            let quality_boost = quality_iter * 10;
            workshop.crafting_fee += quality_boost as u64;
            quality_iter += 1;
        }
        
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitWorkshop<'info> {
    #[account(
        init,
        payer = owner,
        space = 8 + 32 + 64 + 8 + 8 + 1
    )]
    pub workshop: Account<'info, CraftingWorkshop>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct InitMaterialStorage<'info> {
    pub workshop: Account<'info, CraftingWorkshop>,
    #[account(
        init,
        payer = manager,
        space = 8 + 32 + 1 + 4 + 4 + 1
    )]
    pub storage: Account<'info, MaterialStorage>,
    #[account(mut)]
    pub manager: Signer<'info>,
    pub system_program: Program<'info, System>,
}

// 脆弱性: 素材と装備が同じアカウントでも検証されない
#[derive(Accounts)]
pub struct ExecuteCraftingProcess<'info> {
    #[account(mut)]
    pub workshop: Account<'info, CraftingWorkshop>,
    /// CHECK: 素材アカウントの検証が不十分
    pub material_source: AccountInfo<'info>,
    /// CHECK: 装備アカウントの検証が不十分
    pub equipment_target: AccountInfo<'info>,
    pub crafter: Signer<'info>,
}

#[account]
pub struct CraftingWorkshop {
    pub owner: Pubkey,
    pub workshop_name: String,
    pub crafting_fee: u64,
    pub total_crafts: u64,
    pub is_open: bool,
}

#[account]
pub struct MaterialStorage {
    pub workshop: Pubkey,
    pub material_type: MaterialType,
    pub quantity: u32,
    pub quality_grade: u32,
    pub is_available: bool,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq)]
pub enum MaterialType {
    Iron,
    Wood,
    Crystal,
    Leather,
}

use MaterialType::*;

#[error_code]
pub enum CraftingError {
    #[msg("Workshop closed")]
    WorkshopClosed,
    #[msg("Insufficient materials")]
    InsufficientMaterials,
}

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

// ==================== 3. 脆弱なエスクローサービス ====================
// 買い手と売り手の検証が甘く、自己エスクローによる資金操作が可能

use anchor_lang::prelude::*;

declare_id!("V3E4S5C6R7O8W9S0E1R2V3I4C5E6T7R8A9D0E1R2");

#[program]
pub mod vulnerable_escrow_service {
    use super::*;
    
    pub fn init_escrow_agency(
        ctx: Context<InitEscrowAgency>,
        agency_name: String,
        service_rate: u8,
    ) -> Result<()> {
        let agency = &mut ctx.accounts.agency;
        agency.operator = ctx.accounts.operator.key();
        agency.agency_name = agency_name;
        agency.service_rate = service_rate;
        agency.completed_escrows = 0;
        agency.is_accepting = true;
        
        msg!("Escrow agency created: {}", agency.agency_name);
        Ok(())
    }
    
    pub fn init_escrow_contract(
        ctx: Context<InitEscrowContract>,
        contract_value: u64,
        release_condition: ReleaseCondition,
    ) -> Result<()> {
        let contract = &mut ctx.accounts.contract;
        contract.agency = ctx.accounts.agency.key();
        contract.initiator = ctx.accounts.initiator.key();
        contract.contract_value = contract_value;
        contract.release_condition = release_condition;
        contract.escrow_status = EscrowStatus::Pending;
        contract.dispute_count = 0;
        
        msg!("Escrow contract created: {} value", contract_value);
        Ok(())
    }
    
    pub fn execute_escrow_settlement(
        ctx: Context<ExecuteEscrowSettlement>,
        settlement_rounds: u8,
        penalty_rate: u8,
    ) -> Result<()> {
        let agency = &mut ctx.accounts.agency;
        let contract = &mut ctx.accounts.contract;
        
        // 脆弱性: buyer_party と seller_party が同じアカウントでも検証されない
        let mut round = 0u8;
        loop {
            if round >= settlement_rounds { break; }
            
            match contract.escrow_status {
                EscrowStatus::Pending => {
                    // ペンディング時の処理
                    contract.contract_value += (round as u64) * 100;
                    agency.completed_escrows += 1;
                    
                    // 条件判定
                    let condition_met = (round % 3) == 0;
                    if condition_met {
                        contract.escrow_status = EscrowStatus::Released;
                    }
                    
                    msg!("Pending settlement round: {}", round);
                },
                EscrowStatus::Released => {
                    // リリース時の手数料計算
                    let service_fee = (contract.contract_value * agency.service_rate as u64) / 100;
                    contract.contract_value = contract.contract_value.saturating_sub(service_fee);
                    
                    msg!("Released settlement round: {}", round);
                },
                _ => {
                    // その他の状態での調整
                    contract.dispute_count += 1;
                    let penalty = (contract.contract_value * penalty_rate as u64) / 100;
                    contract.contract_value = contract.contract_value.saturating_sub(penalty);
                    
                    msg!("Disputed settlement round: {}", round);
                }
            }
            round += 1;
        }
        
        // 最終決済処理
        let mut final_round = 0u8;
        while final_round < 2 {
            agency.service_rate = (agency.service_rate + final_round + 1).min(50);
            contract.contract_value += final_round as u64 * 50;
            final_round += 1;
        }
        
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitEscrowAgency<'info> {
    #[account(
        init,
        payer = operator,
        space = 8 + 32 + 64 + 1 + 8 + 1
    )]
    pub agency: Account<'info, EscrowAgency>,
    #[account(mut)]
    pub operator: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct InitEscrowContract<'info> {
    pub agency: Account<'info, EscrowAgency>,
    #[account(
        init,
        payer = initiator,
        space = 8 + 32 + 32 + 8 + 1 + 1 + 4
    )]
    pub contract: Account<'info, EscrowContract>,
    #[account(mut)]
    pub initiator: Signer<'info>,
    pub system_program: Program<'info, System>,
}

// 脆弱性: 買い手と売り手が同じアカウントでも検証されない
#[derive(Accounts)]
pub struct ExecuteEscrowSettlement<'info> {
    #[account(mut)]
    pub agency: Account<'info, EscrowAgency>,
    #[account(mut)]
    pub contract: Account<'info, EscrowContract>,
    /// CHECK: 買い手側の検証が不十分
    pub buyer_party: AccountInfo<'info>,
    /// CHECK: 売り手側の検証が不十分
    pub seller_party: AccountInfo<'info>,
    pub settlement_agent: Signer<'info>,
}

#[account]
pub struct EscrowAgency {
    pub operator: Pubkey,
    pub agency_name: String,
    pub service_rate: u8,
    pub completed_escrows: u64,
    pub is_accepting: bool,
}

#[account]
pub struct EscrowContract {
    pub agency: Pubkey,
    pub initiator: Pubkey,
    pub contract_value: u64,
    pub release_condition: ReleaseCondition,
    pub escrow_status: EscrowStatus,
    pub dispute_count: u32,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq)]
pub enum ReleaseCondition {
    TimeBasedRelease,
    ConfirmationBased,
    ArbitratorDecision,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq)]
pub enum EscrowStatus {
    Pending,
    Released,
    Disputed,
    Cancelled,
}

use ReleaseCondition::*;
use EscrowStatus::*;

#[error_code]
pub enum EscrowError {
    #[msg("Contract already settled")]
    AlreadySettled,
    #[msg("Unauthorized release")]
    UnauthorizedRelease,
}