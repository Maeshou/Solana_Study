use anchor_lang::prelude::*;

declare_id!("33333333333333333333333333333334");

#[program]
pub mod battle_arena_program {
    use super::*;

    pub fn initiate_combat_challenge(
        ctx: Context<InitiateCombat>,
        challenger_warrior_id: u32,
        defender_warrior_id: u32,
    ) -> Result<()> {
        let challenger_account = &mut ctx.accounts.challenger_account;
        let defender_account = &mut ctx.accounts.defender_account;
        let battle_session = &mut ctx.accounts.battle_session;
        
        require!(
            challenger_account.warrior_tokens.contains(&challenger_warrior_id),
            BattleError::WarriorNotOwned
        );
        
        require!(
            defender_account.warrior_tokens.contains(&defender_warrior_id),
            BattleError::DefenderWarriorInvalid
        );
        
        let challenger_power = calculate_warrior_combat_rating(challenger_warrior_id);
        let defender_power = calculate_warrior_combat_rating(defender_warrior_id);
        
        battle_session.challenger_address = ctx.accounts.challenger.key();
        battle_session.defender_address = ctx.accounts.defender.key();
        battle_session.challenger_warrior = challenger_warrior_id;
        battle_session.defender_warrior = defender_warrior_id;
        battle_session.battle_status = BattleStatus::InProgress;
        battle_session.combat_rounds_remaining = 10;
        battle_session.challenger_health = 1000;
        battle_session.defender_health = 1000;
        
        let initial_damage = challenger_power / 20;
        battle_session.defender_health = battle_session.defender_health.saturating_sub(initial_damage);
        
        emit!(CombatInitiated {
            battle_id: battle_session.key(),
            challenger: ctx.accounts.challenger.key(),
            defender: ctx.accounts.defender.key(),
        });
        
        Ok(())
    }
    
    pub fn execute_combat_round(
        ctx: Context<ExecuteCombatRound>,
        attack_strategy: AttackStrategy,
    ) -> Result<()> {
        let battle_session = &mut ctx.accounts.battle_session;
        let attacker_account = &ctx.accounts.attacker_account;
        
        require!(
            battle_session.battle_status == BattleStatus::InProgress,
            BattleError::BattleNotActive
        );
        
        require!(
            battle_session.combat_rounds_remaining > 0,
            BattleError::BattleAlreadyFinished
        );
        
        let attacker_warrior_id = if ctx.accounts.attacker.key() == battle_session.challenger_address {
            battle_session.challenger_warrior
        } else {
            battle_session.defender_warrior
        };
        
        let base_attack_damage = calculate_attack_damage(attacker_warrior_id, attack_strategy);
        let critical_hit_chance = attacker_warrior_id % 20;
        
        let final_damage = if critical_hit_chance > 15 {
            base_attack_damage * 2 // Critical hit
        } else {
            base_attack_damage
        };
        
        if ctx.accounts.attacker.key() == battle_session.challenger_address {
            battle_session.defender_health = battle_session.defender_health.saturating_sub(final_damage);
        } else {
            battle_session.challenger_health = battle_session.challenger_health.saturating_sub(final_damage);
        }
        
        battle_session.combat_rounds_remaining -= 1;
        
        // Check victory conditions
        let victory_condition = battle_session.challenger_health > 0 && battle_session.defender_health > 0;
        if !victory_condition || battle_session.combat_rounds_remaining <= 0 {
            battle_session.battle_status = BattleStatus::Completed;
            
            let winner_address = if battle_session.challenger_health > battle_session.defender_health {
                battle_session.challenger_address
            } else {
                battle_session.defender_address
            };
            
            emit!(BattleCompleted {
                battle_id: battle_session.key(),
                winner: winner_address,
                final_damage: final_damage,
            });
        }
        
        Ok(())
    }
    
    pub fn upgrade_warrior_equipment(
        ctx: Context<UpgradeEquipment>,
        warrior_id: u32,
        equipment_type: EquipmentType,
        upgrade_level: u8,
    ) -> Result<()> {
        let warrior_owner = &mut ctx.accounts.warrior_owner;
        let equipment_forge = &ctx.accounts.equipment_forge;
        
        require!(
            warrior_owner.warrior_tokens.contains(&warrior_id),
            BattleError::WarriorNotOwned
        );
        
        let upgrade_cost_calculation = (upgrade_level as u64) * 500;
        let equipment_multiplier = match equipment_type {
            EquipmentType::Weapon => 3,
            EquipmentType::Armor => 2,
            EquipmentType::Accessory => 1,
        };
        
        let total_upgrade_cost = upgrade_cost_calculation * equipment_multiplier;
        
        require!(
            warrior_owner.battle_tokens >= total_upgrade_cost,
            BattleError::InsufficientUpgradeFunds
        );
        
        warrior_owner.battle_tokens -= total_upgrade_cost;
        
        let equipment_bonus = upgrade_level as u32 * 25;
        warrior_owner.total_warrior_power += equipment_bonus;
        
        let upgraded_equipment_id = warrior_id + (equipment_bonus * 1000);
        warrior_owner.equipment_inventory.push(upgraded_equipment_id);
        
        Ok(())
    }
}

fn calculate_warrior_combat_rating(warrior_id: u32) -> u32 {
    let base_power = warrior_id % 1000;
    let rarity_bonus = warrior_id / 10000;
    base_power + (rarity_bonus * 50)
}

fn calculate_attack_damage(warrior_id: u32, strategy: AttackStrategy) -> u32 {
    let base_damage = warrior_id % 100;
    let strategy_modifier = match strategy {
        AttackStrategy::Aggressive => base_damage + 20,
        AttackStrategy::Defensive => base_damage + 5,
        AttackStrategy::Balanced => base_damage + 12,
    };
    strategy_modifier
}

#[derive(Accounts)]
pub struct InitiateCombat<'info> {
    #[account(mut)]
    pub challenger_account: Account<'info, WarriorOwnerAccount>,
    
    pub defender_account: Account<'info, WarriorOwnerAccount>,
    
    #[account(
        init,
        payer = challenger,
        space = 8 + BattleSession::INIT_SPACE
    )]
    pub battle_session: Account<'info, BattleSession>,
    
    #[account(mut)]
    pub challenger: Signer<'info>,
    
    /// CHECK: This is the defender's public key
    pub defender: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ExecuteCombatRound<'info> {
    #[account(mut)]
    pub battle_session: Account<'info, BattleSession>,
    
    pub attacker_account: Account<'info, WarriorOwnerAccount>,
    
    pub attacker: Signer<'info>,
}

#[derive(Accounts)]
pub struct UpgradeEquipment<'info> {
    #[account(mut)]
    pub warrior_owner: Account<'info, WarriorOwnerAccount>,
    
    pub equipment_forge: Account<'info, EquipmentForge>,
    
    pub authority: Signer<'info>,
}

#[account]
#[derive(InitSpace)]
pub struct WarriorOwnerAccount {
    pub owner_address: Pubkey,
    #[max_len(20)]
    pub warrior_tokens: Vec<u32>,
    pub battle_tokens: u64,
    pub total_warrior_power: u32,
    #[max_len(50)]
    pub equipment_inventory: Vec<u32>,
    pub arena_victories: u32,
}

#[account]
#[derive(InitSpace)]
pub struct BattleSession {
    pub challenger_address: Pubkey,
    pub defender_address: Pubkey,
    pub challenger_warrior: u32,
    pub defender_warrior: u32,
    pub battle_status: BattleStatus,
    pub combat_rounds_remaining: u8,
    pub challenger_health: u32,
    pub defender_health: u32,
}

#[account]
#[derive(InitSpace)]
pub struct EquipmentForge {
    pub forge_level: u32,
    pub upgrade_success_rate: u8,
    pub master_craftsman: Pubkey,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq, InitSpace)]
pub enum BattleStatus {
    Pending,
    InProgress,
    Completed,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace)]
pub enum AttackStrategy {
    Aggressive,
    Defensive,
    Balanced,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace)]
pub enum EquipmentType {
    Weapon,
    Armor,
    Accessory,
}

#[event]
pub struct CombatInitiated {
    pub battle_id: Pubkey,
    pub challenger: Pubkey,
    pub defender: Pubkey,
}

#[event]
pub struct BattleCompleted {
    pub battle_id: Pubkey,
    pub winner: Pubkey,
    pub final_damage: u32,
}

#[error_code]
pub enum BattleError {
    #[msg("Warrior not owned by challenger")]
    WarriorNotOwned,
    #[msg("Invalid defender warrior")]
    DefenderWarriorInvalid,
    #[msg("Battle is not currently active")]
    BattleNotActive,
    #[msg("Battle has already finished")]
    BattleAlreadyFinished,
    #[msg("Insufficient tokens for equipment upgrade")]
    InsufficientUpgradeFunds,
}