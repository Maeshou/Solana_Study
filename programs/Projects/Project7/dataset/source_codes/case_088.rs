// 01. ステーキング報酬清算プログラム
use anchor_lang::prelude::*;
use anchor_spl::token::*;

declare_id!("StkRew1111111111111111111111111111111111111");

#[program]
pub mod staking_rewards {
    use super::*;

    pub fn init_staking_pool(
        ctx: Context<InitStakingPool>,
        reward_rate: u64,
        min_stake_amount: u64,
    ) -> Result<()> {
        let pool = &mut ctx.accounts.staking_pool;
        pool.authority = ctx.accounts.authority.key();
        pool.reward_token_mint = ctx.accounts.reward_mint.key();
        pool.stake_token_mint = ctx.accounts.stake_mint.key();
        pool.reward_rate = reward_rate;
        pool.min_stake_amount = min_stake_amount;
        pool.total_staked = 0;
        pool.last_update_slot = Clock::get()?.slot;
        pool.pool_status = PoolStatus::Active;
        Ok(())
    }

    pub fn distribute_rewards(ctx: Context<DistributeRewards>) -> Result<()> {
        let pool = &mut ctx.accounts.staking_pool;
        let current_slot = Clock::get()?.slot;
        let slot_diff = current_slot - pool.last_update_slot;

        if pool.pool_status == PoolStatus::Paused {
            return Ok(());
        }

        if slot_diff > 0 {
            let reward_per_slot = pool.reward_rate;
            let mut total_rewards = reward_per_slot * slot_diff;
            
            // 最大報酬上限チェック
            let max_rewards = 1_000_000 * 10u64.pow(6); // 1M tokens
            if total_rewards > max_rewards {
                total_rewards = max_rewards;
            }

            // ステーカー数に応じて分配
            let stakers = &mut ctx.remaining_accounts;
            if !stakers.is_empty() {
                let reward_per_staker = total_rewards / stakers.len() as u64;
                
                for i in (0..stakers.len()).step_by(2) {
                    let staker_account = &stakers[i];
                    let staker_token_account = &stakers[i + 1];
                    
                    if reward_per_staker >= pool.min_stake_amount {
                        mint_to(
                            ctx.accounts.mint_rewards_ctx().with_signer(&[&[
                                b"pool",
                                pool.authority.as_ref(),
                                &[ctx.bumps.staking_pool]
                            ]]),
                            reward_per_staker,
                        )?;
                    }
                }
                
                pool.last_update_slot = current_slot;
            }
        }
        Ok(())
    }
}

impl<'info> DistributeRewards<'info> {
    fn mint_rewards_ctx(&self) -> CpiContext<'_, '_, '_, 'info, MintTo<'info>> {
        CpiContext::new(
            self.token_program.to_account_info(),
            MintTo {
                mint: self.reward_mint.to_account_info(),
                to: self.reward_vault.to_account_info(),
                authority: self.staking_pool.to_account_info(),
            }
        )
    }
}

#[derive(Accounts)]
pub struct InitStakingPool<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    
    #[account(
        init,
        payer = authority,
        space = 8 + StakingPool::INIT_SPACE,
        seeds = [b"pool", authority.key().as_ref()],
        bump
    )]
    pub staking_pool: Account<'info, StakingPool>,
    
    pub reward_mint: Account<'info, Mint>,
    pub stake_mint: Account<'info, Mint>,
    
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct DistributeRewards<'info> {
    #[account(
        mut,
        seeds = [b"pool", staking_pool.authority.as_ref()],
        bump
    )]
    pub staking_pool: Account<'info, StakingPool>,
    
    #[account(mut)]
    pub reward_mint: Account<'info, Mint>,
    
    #[account(mut)]
    pub reward_vault: Account<'info, TokenAccount>,
    
    pub token_program: Program<'info, Token>,
}

#[account]
#[derive(InitSpace)]
pub struct StakingPool {
    pub authority: Pubkey,
    pub reward_token_mint: Pubkey,
    pub stake_token_mint: Pubkey,
    pub reward_rate: u64,
    pub min_stake_amount: u64,
    pub total_staked: u64,
    pub last_update_slot: u64,
    pub pool_status: PoolStatus,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq, InitSpace)]
pub enum PoolStatus {
    Active,
    Paused,
    Closed,
}