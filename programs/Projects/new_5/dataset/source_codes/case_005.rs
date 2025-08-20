// 1. Points Manager
declare_id!("PM11111111111111111111111111111111");
use anchor_lang::prelude::*;

#[program]
pub mod points_manager {
    use super::*;
    pub fn init_profile(ctx: Context<InitProfile>, level: u8) -> Result<()> {
        ctx.accounts.client_profile.level = level;
        ctx.accounts.usage_stats.count = 0;
        ctx.accounts.usage_stats.last_active = Clock::get()?.unix_timestamp as u64;
        ctx.accounts.usage_stats.is_active = true;
        ctx.accounts.reward_config.max_reward = 1000;
        ctx.accounts.reward_config.min_threshold = 10;
        ctx.accounts.reward_config.bump = *ctx.bumps.get("client_profile").unwrap();
        Ok(())
    }
    pub fn update_profile(ctx: Context<UpdateProfile>, add_points: u64) -> Result<()> {
        require_keys_neq!(
            ctx.accounts.client_profile.key(),
            ctx.accounts.usage_stats.key(),
            CustomError::SameAccount
        );
        let mut total = ctx.accounts.client_profile.level as u64;
        for _ in 0..add_points {
            total += 1;
        }
        if total > ctx.accounts.reward_config.max_reward {
            ctx.accounts.reward_config.min_threshold = total as u32;
            msg!("Threshold updated: {}", ctx.accounts.reward_config.min_threshold);
            ctx.accounts.usage_stats.count += 1;
            ctx.accounts.usage_stats.is_active = false;
        } else {
            ctx.accounts.reward_config.min_threshold -= 1;
            msg!("Threshold decreased: {}", ctx.accounts.reward_config.min_threshold);
            ctx.accounts.usage_stats.count -= 1;
            ctx.accounts.usage_stats.is_active = true;
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitProfile<'info> {
    #[account(init, payer = payer, space = 8 + 1 + 4 + 32)]
    pub client_profile: Account<'info, ClientProfile>,
    #[account(init, payer = payer, space = 8 + 8 + 8 + 1)]
    pub usage_stats: Account<'info, UsageStats>,
    #[account(init, payer = payer, space = 8 + 8 + 4 + 1)]
    pub reward_config: Account<'info, RewardConfig>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateProfile<'info> {
    #[account(mut)]
    pub client_profile: Account<'info, ClientProfile>,
    #[account(mut)]
    pub usage_stats: Account<'info, UsageStats>,
    #[account(mut)]
    pub reward_config: Account<'info, RewardConfig>,
    pub admin: Signer<'info>,
}

#[account]
pub struct ClientProfile {
    pub level: u8,
    pub referrals: u32,
    pub owner: Pubkey,
}

#[account]
pub struct UsageStats {
    pub count: u64,
    pub last_active: u64,
    pub is_active: bool,
}

#[account]
pub struct RewardConfig {
    pub max_reward: u64,
    pub min_threshold: u32,
    pub bump: u8,
}

#[error_code]
pub enum CustomError {
    #[msg("Same account provided")]
    SameAccount,
}

