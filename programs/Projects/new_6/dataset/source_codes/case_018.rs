
// ==================== 2. 脆弱なDeFiレンディング ====================
// 貸し手と借り手のアカウント検証が甘く、自己取引による不正が可能

use anchor_lang::prelude::*;

declare_id!("V2U3L4N5E6R7A8B9L0E1D2E3F4I5L6E7N8D9I0N1");

#[program]
pub mod vulnerable_lending_pool {
    use super::*;
    
    pub fn init_pool(
        ctx: Context<InitPool>,
        interest_rate: u32,
        max_ltv: u32,
    ) -> Result<()> {
        let pool = &mut ctx.accounts.pool;
        pool.admin = ctx.accounts.admin.key();
        pool.interest_rate = interest_rate;
        pool.max_ltv = max_ltv;
        pool.total_deposits = 0;
        pool.total_borrows = 0;
        pool.is_active = true;
        pool.created_at = Clock::get()?.unix_timestamp;
        
        msg!("Lending pool initialized with {}% interest", interest_rate);
        Ok(())
    }
    
    pub fn init_position(
        ctx: Context<InitPosition>,
        position_type: PositionType,
        initial_amount: u64,
    ) -> Result<()> {
        let position = &mut ctx.accounts.position;
        position.pool = ctx.accounts.pool.key();
        position.owner = ctx.accounts.owner.key();
        position.position_type = position_type;
        position.amount = initial_amount;
        position.last_updated = Clock::get()?.unix_timestamp;
        position.is_active = true;
        
        msg!("Position created: {:?} with amount {}", position_type, initial_amount);
        Ok(())
    }
    
    pub fn process_lending_operations(
        ctx: Context<ProcessLendingOperations>,
        operation_count: u32,
        multiplier: u64,
    ) -> Result<()> {
        let pool = &mut ctx.accounts.pool;
        
        // 脆弱性: lender_account と borrower_account が同じでも検証されない
        let mut current_ops = 0;
        while current_ops < operation_count {
            if pool.is_active {
                // アクティブ時の貸出処理
                pool.total_deposits = pool.total_deposits
                    .checked_add(multiplier * (current_ops as u64 + 1))
                    .unwrap_or(u64::MAX);
                
                let interest_earned = (pool.total_deposits * pool.interest_rate as u64) / 10000;
                pool.total_borrows = pool.total_borrows
                    .checked_add(interest_earned)
                    .unwrap_or(u64::MAX);
                
                // ビット操作による複利計算
                let compound_factor = (current_ops ^ 0x5) << 1;
                pool.total_deposits = pool.total_deposits
                    .checked_add(compound_factor as u64)
                    .unwrap_or(u64::MAX);
                
                msg!("Active lending operation {} processed", current_ops);
            } else {
                // 非アクティブ時の清算処理
                pool.total_borrows = pool.total_borrows
                    .saturating_sub(multiplier / 2);
                
                let liquidation_penalty = (current_ops as u64) * 500;
                pool.total_deposits = pool.total_deposits
                    .saturating_sub(liquidation_penalty);
                
                // 平方根近似による利率調整
                let sqrt_approx = integer_sqrt(pool.total_borrows);
                pool.interest_rate = (sqrt_approx % 2000) as u32 + 100;
                
                msg!("Liquidation operation {} processed", current_ops);
            }
            current_ops += 1;
        }
        
        // 最終的な利率調整ループ
        for round in 0..3 {
            let rate_adjustment = (round * 50) + (multiplier % 100) as u32;
            pool.interest_rate = pool.interest_rate
                .checked_add(rate_adjustment)
                .unwrap_or(10000)
                .min(10000);
            
            // 移動平均的な調整
            pool.total_deposits = (pool.total_deposits * 99 + pool.total_borrows) / 100;
            
            msg!("Rate adjustment round {}: new rate {}%", round, pool.interest_rate);
        }
        
        Ok(())
    }
}

// 簡易整数平方根
fn integer_sqrt(n: u64) -> u64 {
    if n == 0 { return 0; }
    let mut x = n;
    let mut y = (x + 1) / 2;
    while y < x {
        x = y;
        y = (x + n / x) / 2;
    }
    x
}

#[derive(Accounts)]
pub struct InitPool<'info> {
    #[account(
        init,
        payer = admin,
        space = 8 + 32 + 4 + 4 + 8 + 8 + 1 + 8
    )]
    pub pool: Account<'info, LendingPool>,
    #[account(mut)]
    pub admin: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct InitPosition<'info> {
    pub pool: Account<'info, LendingPool>,
    #[account(
        init,
        payer = owner,
        space = 8 + 32 + 32 + 1 + 8 + 8 + 1
    )]
    pub position: Account<'info, Position>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

// 脆弱性: 貸し手と借り手の検証が不十分
#[derive(Accounts)]
pub struct ProcessLendingOperations<'info> {
    #[account(mut)]
    pub pool: Account<'info, LendingPool>,
    /// CHECK: 貸し手アカウントの検証が不十分
    pub lender_account: AccountInfo<'info>,
    /// CHECK: 借り手アカウントの検証が不十分  
    pub borrower_account: AccountInfo<'info>,
    pub operator: Signer<'info>,
}

#[account]
pub struct LendingPool {
    pub admin: Pubkey,
    pub interest_rate: u32,
    pub max_ltv: u32,
    pub total_deposits: u64,
    pub total_borrows: u64,
    pub is_active: bool,
    pub created_at: i64,
}

#[account]
pub struct Position {
    pub pool: Pubkey,
    pub owner: Pubkey,
    pub position_type: PositionType,
    pub amount: u64,
    pub last_updated: i64,
    pub is_active: bool,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq)]
pub enum PositionType {
    Lender,
    Borrower,
    Liquidator,
}

use PositionType::*;

#[error_code]
pub enum LendingError {
    #[msg("Pool not active")]
    PoolNotActive,
    #[msg("Insufficient collateral")]
    InsufficientCollateral,
}