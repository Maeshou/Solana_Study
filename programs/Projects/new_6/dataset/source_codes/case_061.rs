// 18. Yield Farming - Farm Owner vs Farmer confusion
use anchor_lang::prelude::*;

declare_id!("YieldFarming888888888888888888888888888888888");

#[program]
pub mod yield_farming {
    use super::*;

    pub fn init_yield_farm(ctx: Context<InitYieldFarm>, reward_token_per_block: u64, bonus_multiplier: u16) -> Result<()> {
        let farm = &mut ctx.accounts.yield_farm;
        farm.farm_owner = ctx.accounts.owner.key();
        farm.reward_per_block = reward_token_per_block;
        farm.bonus_multiplier = bonus_multiplier;
        farm.total_staked = 0;
        farm.total_rewards_distributed = 0;
        farm.start_block = Clock::get()?.slot;
        farm.last_reward_block = Clock::get()?.slot;
        farm.accumulated_reward_per_share = 0;
        farm.emission_rate = 100; // Base emission rate
        Ok(())
    }

    pub fn harvest_rewards(ctx: Context<HarvestRewards>, user_address: Pubkey, force_compound: bool) -> Result<()> {
        let farm = &mut ctx.accounts.yield_farm;
        let user_info = &mut ctx.accounts.user_farming_info;
        let harvester = &ctx.accounts.harvester;
        
        // Vulnerable: Any account can harvest rewards for any user
        let current_block = Clock::get()?.slot;
        let blocks_elapsed = current_block - farm.last_reward_block;
        
        // Update farm reward calculations
        if blocks_elapsed > 0 && farm.total_staked > 0 {
            let reward = blocks_elapsed * farm.reward_per_block;
            farm.accumulated_reward_per_share += (reward * 1e12 as u64) / farm.total_staked;
            farm.last_reward_block = current_block;
        }
        
        // Calculate pending rewards for user
        let user_accumulated_reward = (user_info.staked_amount * farm.accumulated_reward_per_share) / 1e12 as u64;
        let pending_reward = user_accumulated_reward - user_info.reward_debt;
        
        if pending_reward > 0 {
            user_info.total_harvested += pending_reward;
            farm.total_rewards_distributed += pending_reward;
            
            // Complex reward distribution logic
            if force_compound {
                // Auto-compound rewards back into staking
                user_info.staked_amount += pending_reward;
                farm.total_staked += pending_reward;
                user_info.compound_count += 1;
                
                // Bonus for compounding
                let compound_bonus = (pending_reward * farm.bonus_multiplier as u64) / 10000;
                user_info.bonus_rewards += compound_bonus;
                farm.total_bonus_distributed += compound_bonus;
            }
            
            // Update boost multipliers based on farming history
            let farming_duration = current_block - user_info.first_stake_block;
            if farming_duration > 100000 { // Long-term farming bonus
                user_info.loyalty_multiplier = (user_info.loyalty_multiplier + 5).min(200); // Max 2x
            }
            
            // Apply time-based multipliers
            for week in 0..4 {
                if farming_duration > (week + 1) as u64 * 10080 { // Blocks per week
                    user_info.weekly_multipliers[week] = 110 + (week as u16 * 10);
                }
            }
            
            user_info.reward_debt = (user_info.staked_amount * farm.accumulated_reward_per_share) / 1e12 as u64;
        }
        
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitYieldFarm<'info> {
    #[account(init, payer = owner, space = 8 + 400)]
    pub yield_farm: Account<'info, YieldFarmData>,
    #[account(mut)]
    pub owner: AccountInfo<'info>, // No farm owner verification
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct HarvestRewards<'info> {
    #[account(mut)]
    pub yield_farm: Account<'info, YieldFarmData>,
    #[account(mut)]
    pub user_farming_info: Account<'info, UserFarmingInfo>,
    pub harvester: AccountInfo<'info>, // Could harvest for anyone, not just the user
}

#[account]
pub struct YieldFarmData {
    pub farm_owner: Pubkey,
    pub reward_per_block: u64,
    pub bonus_multiplier: u16,
    pub total_staked: u64,
    pub total_rewards_distributed: u64,
    pub total_bonus_distributed: u64,
    pub start_block: u64,
    pub last_reward_block: u64,
    pub accumulated_reward_per_share: u64,
    pub emission_rate: u16,
}

#[account]
pub struct UserFarmingInfo {
    pub farmer: Pubkey,
    pub staked_amount: u64,
    pub reward_debt: u64,
    pub total_harvested: u64,
    pub bonus_rewards: u64,
    pub compound_count: u32,
    pub first_stake_block: u64,
    pub loyalty_multiplier: u16,
    pub weekly_multipliers: [u16; 4],
}
