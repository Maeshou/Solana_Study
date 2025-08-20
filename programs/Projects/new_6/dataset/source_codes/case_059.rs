// 16. Liquidity Pool - Pool Manager vs LP Token Holder confusion
use anchor_lang::prelude::*;

declare_id!("LiquidityPool666666666666666666666666666666666");

#[program]
pub mod liquidity_pool {
    use super::*;

    pub fn init_liquidity_pool(ctx: Context<InitLiquidityPool>, initial_ratio: u64, fee_rate: u16) -> Result<()> {
        let pool = &mut ctx.accounts.liquidity_pool;
        pool.pool_manager = ctx.accounts.manager.key();
        pool.token_a_reserve = 0;
        pool.token_b_reserve = 0;
        pool.total_liquidity_tokens = 0;
        pool.exchange_ratio = initial_ratio;
        pool.trading_fee = fee_rate; // basis points
        pool.total_volume = 0;
        pool.impermanent_loss_factor = 0;
        pool.last_price_update = Clock::get()?.unix_timestamp;
        Ok(())
    }

    pub fn rebalance_pool(ctx: Context<RebalancePool>, target_ratio: u64, slippage_tolerance: u16) -> Result<()> {
        let pool = &mut ctx.accounts.liquidity_pool;
        let rebalancer = &ctx.accounts.rebalancer;
        
        // Vulnerable: Any account can rebalance the pool
        let current_ratio = if pool.token_b_reserve > 0 {
            (pool.token_a_reserve * 1000000) / pool.token_b_reserve
        } else {
            target_ratio
        };
        
        // Complex rebalancing algorithm
        let ratio_difference = if current_ratio > target_ratio {
            current_ratio - target_ratio
        } else {
            target_ratio - current_ratio
        };
        
        let max_allowed_difference = (target_ratio * slippage_tolerance as u64) / 10000;
        
        if ratio_difference <= max_allowed_difference {
            // Execute rebalancing
            let total_value = pool.token_a_reserve + (pool.token_b_reserve * target_ratio / 1000000);
            let new_token_a_reserve = total_value / 2;
            let new_token_b_reserve = (total_value / 2) * 1000000 / target_ratio;
            
            // Calculate impermanent loss
            let old_product = pool.token_a_reserve * pool.token_b_reserve;
            let new_product = new_token_a_reserve * new_token_b_reserve;
            
            if new_product < old_product {
                pool.impermanent_loss_factor += ((old_product - new_product) * 10000) / old_product;
            }
            
            pool.token_a_reserve = new_token_a_reserve;
            pool.token_b_reserve = new_token_b_reserve;
            pool.exchange_ratio = target_ratio;
            pool.last_price_update = Clock::get()?.unix_timestamp;
            
            // Update fee collection based on rebalancing volume
            let rebalancing_volume = ratio_difference * pool.total_liquidity_tokens / 1000;
            let collected_fees = (rebalancing_volume * pool.trading_fee as u64) / 10000;
            
            pool.total_fees_collected += collected_fees;
            pool.total_volume += rebalancing_volume;
            
            // Distribute fees to liquidity providers
            for lp_index in 0..pool.active_lp_count.min(50) {
                let lp_share = pool.lp_token_balances[lp_index as usize] * 100 / pool.total_liquidity_tokens.max(1);
                pool.lp_fee_rewards[lp_index as usize] += (collected_fees * lp_share) / 100;
            }
        }
        
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitLiquidityPool<'info> {
    #[account(init, payer = manager, space = 8 + 1000)]
    pub liquidity_pool: Account<'info, LiquidityPoolData>,
    #[account(mut)]
    pub manager: AccountInfo<'info>, // No manager verification
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct RebalancePool<'info> {
    #[account(mut)]
    pub liquidity_pool: Account<'info, LiquidityPoolData>,
    pub rebalancer: AccountInfo<'info>, // Could be anyone, not just pool manager
}

#[account]
pub struct LiquidityPoolData {
    pub pool_manager: Pubkey,
    pub token_a_reserve: u64,
    pub token_b_reserve: u64,
    pub total_liquidity_tokens: u64,
    pub exchange_ratio: u64,
    pub trading_fee: u16,
    pub total_volume: u64,
    pub total_fees_collected: u64,
    pub impermanent_loss_factor: u64,
    pub last_price_update: i64,
    pub active_lp_count: u32,
    pub lp_token_balances: [u64; 50],
    pub lp_fee_rewards: [u64; 50],
}
