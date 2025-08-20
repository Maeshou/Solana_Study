use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount, Mint};

declare_id!("22222222222222222222222222222222");

#[program]
pub mod guild_territory_management {
    use super::*;

    pub fn establish_guild_territory(
        ctx: Context<EstablishTerritory>,
        guild_name: String,
        territory_coordinates: TerritoryCoordinates,
        initial_defense_budget: u64,
    ) -> Result<()> {
        let guild_account = &mut ctx.accounts.guild_account;
        let current_time = Clock::get()?.unix_timestamp;
        
        guild_account.guild_master = ctx.accounts.guild_master.key();
        guild_account.name = guild_name;
        guild_account.establishment_date = current_time;
        guild_account.member_count = 1;
        guild_account.territory = territory_coordinates;
        guild_account.treasury_balance = initial_defense_budget;
        
        let territory_size = territory_coordinates.calculate_area();
        require!(territory_size >= 100, GuildError::TerritoryTooSmall);
        require!(territory_size <= 10000, GuildError::TerritoryTooLarge);
        
        let mut defense_structures = Vec::new();
        let structure_count = match territory_size {
            size if size >= 5000 => {
                guild_account.max_member_capacity = 50;
                guild_account.daily_resource_generation = 1000;
                guild_account.territory_tier = TerritoryTier::Capital;
                
                for structure_id in 0..8 {
                    defense_structures.push(DefenseStructure {
                        structure_id,
                        structure_type: match structure_id % 4 {
                            0 => StructureType::Watchtower,
                            1 => StructureType::BarricadeWall,
                            2 => StructureType::MagicBarrier,
                            _ => StructureType::TrapField,
                        },
                        health_points: 2000,
                        defense_power: 300,
                        maintenance_cost: 50,
                        last_upgrade: current_time,
                    });
                }
                8
            },
            size if size >= 2000 => {
                guild_account.max_member_capacity = 30;
                guild_account.daily_resource_generation = 500;
                guild_account.territory_tier = TerritoryTier::City;
                
                for structure_id in 0..5 {
                    defense_structures.push(DefenseStructure {
                        structure_id,
                        structure_type: match structure_id % 3 {
                            0 => StructureType::Watchtower,
                            1 => StructureType::BarricadeWall,
                            _ => StructureType::MagicBarrier,
                        },
                        health_points: 1500,
                        defense_power: 200,
                        maintenance_cost: 30,
                        last_upgrade: current_time,
                    });
                }
                5
            },
            _ => {
                guild_account.max_member_capacity = 15;
                guild_account.daily_resource_generation = 200;
                guild_account.territory_tier = TerritoryTier::Outpost;
                
                for structure_id in 0..3 {
                    defense_structures.push(DefenseStructure {
                        structure_id,
                        structure_type: match structure_id {
                            0 => StructureType::Watchtower,
                            1 => StructureType::BarricadeWall,
                            _ => StructureType::TrapField,
                        },
                        health_points: 1000,
                        defense_power: 150,
                        maintenance_cost: 20,
                        last_upgrade: current_time,
                    });
                }
                3
            }
        };
        
        guild_account.defense_structures = defense_structures;
        
        let mut resource_nodes = Vec::new();
        let available_resources = [
            ResourceType::Gold,
            ResourceType::Crystal,
            ResourceType::Wood,
            ResourceType::Stone,
            ResourceType::MagicEssence
        ];
        
        for (node_index, resource_type) in available_resources.iter().enumerate() {
            let generation_rate = match guild_account.territory_tier {
                TerritoryTier::Capital => 100 + (node_index as u32 * 20),
                TerritoryTier::City => 60 + (node_index as u32 * 15),
                TerritoryTier::Outpost => 30 + (node_index as u32 * 10),
            };
            
            resource_nodes.push(ResourceNode {
                resource_type: resource_type.clone(),
                current_amount: 0,
                generation_rate,
                capacity: generation_rate * 24,
                last_harvest: current_time,
                enhancement_level: 0,
            });
        }
        guild_account.resource_nodes = resource_nodes;
        
        guild_account.alliance_relationships = Vec::new();
        guild_account.war_declarations = Vec::new();
        guild_account.territory_disputes = Vec::new();
        
        guild_account.guild_bonuses = GuildBonuses {
            experience_multiplier: 110,
            resource_bonus: 105,
            defense_bonus: 100,
            attack_bonus: 100,
        };
        
        Ok(())
    }
}

#[derive(Accounts)]
pub struct EstablishTerritory<'info> {
    #[account(
        init,
        payer = guild_master,
        space = 8 + GuildAccount::LEN,
        seeds = [b"guild", guild_master.key().as_ref()],
        bump
    )]
    pub guild_account: Account<'info, GuildAccount>,
    
    #[account(mut)]
    pub guild_master: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}

#[account]
pub struct GuildAccount {
    pub guild_master: Pubkey,
    pub name: String,
    pub establishment_date: i64,
    pub member_count: u32,
    pub max_member_capacity: u32,
    pub territory: TerritoryCoordinates,
    pub territory_tier: TerritoryTier,
    pub treasury_balance: u64,
    pub daily_resource_generation: u32,
    pub defense_structures: Vec<DefenseStructure>,
    pub resource_nodes: Vec<ResourceNode>,
    pub alliance_relationships: Vec<AllianceRecord>,
    pub war_declarations: Vec<WarDeclaration>,
    pub territory_disputes: Vec<TerritoryDispute>,
    pub guild_bonuses: GuildBonuses,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct TerritoryCoordinates {
    pub north_boundary: i32,
    pub south_boundary: i32,
    pub east_boundary: i32,
    pub west_boundary: i32,
}

impl TerritoryCoordinates {
    pub fn calculate_area(&self) -> u32 {
        let width = (self.east_boundary - self.west_boundary).abs() as u32;
        let height = (self.north_boundary - self.south_boundary).abs() as u32;
        width * height
    }
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq)]
pub enum TerritoryTier {
    Outpost,
    City,
    Capital,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct DefenseStructure {
    pub structure_id: u32,
    pub structure_type: StructureType,
    pub health_points: u32,
    pub defense_power: u32,
    pub maintenance_cost: u32,
    pub last_upgrade: i64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub enum StructureType {
    Watchtower,
    BarricadeWall,
    MagicBarrier,
    TrapField,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct ResourceNode {
    pub resource_type: ResourceType,
    pub current_amount: u32,
    pub generation_rate: u32,
    pub capacity: u32,
    pub last_harvest: i64,
    pub enhancement_level: u8,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub enum ResourceType {
    Gold,
    Crystal,
    Wood,
    Stone,
    MagicEssence,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct AllianceRecord {
    pub allied_guild: Pubkey,
    pub alliance_type: AllianceType,
    pub established_date: i64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub enum AllianceType {
    Trade,
    Military,
    NonAggression,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct WarDeclaration {
    pub enemy_guild: Pubkey,
    pub declaration_date: i64,
    pub war_reason: String,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct TerritoryDispute {
    pub disputed_coordinates: TerritoryCoordinates,
    pub disputing_guild: Pubkey,
    pub dispute_start: i64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct GuildBonuses {
    pub experience_multiplier: u32,
    pub resource_bonus: u32,
    pub defense_bonus: u32,
    pub attack_bonus: u32,
}

impl GuildAccount {
    pub const LEN: usize = 32 + 4 + 32 + 8 + 4 + 4 + 32 + 1 + 8 + 4 + 500 + 300 + 200 + 150 + 100 + 32;
}

#[error_code]
pub enum GuildError {
    #[msg("Territory size is too small")]
    TerritoryTooSmall,
    #[msg("Territory size exceeds maximum allowed")]
    TerritoryTooLarge,
}