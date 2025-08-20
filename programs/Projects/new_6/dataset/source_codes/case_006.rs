// ========================================
// 6. 脆弱なレンディングプール - Vulnerable Lending Pool
// ========================================

use anchor_lang::prelude::*;

declare_id!("V6uLnErAbLeCoD3F0r3xAmP1e5tUdY7BaTt1eAr3nA5x");

#[program]
pub mod vulnerable_lending {
    use super::*;

    pub fn init_lending_pool(ctx: Context<InitLendingPool>) -> Result<()> {
        let pool = &mut ctx.accounts.lending_pool;
        pool.manager = ctx.accounts.manager.key();
        pool.total_deposits = 0;
        pool.total_borrows = 0;
        pool.interest_rate = 8; // 8% APR
        Ok(())
    }

    pub fn create_deposit(ctx: Context<CreateDeposit>, amount: u64) -> Result<()> {
        let deposit = &mut ctx.accounts.deposit_account;
        deposit.pool = ctx.accounts.lending_pool.key();
        deposit.depositor = ctx.accounts.depositor.key();
        deposit.amount = amount;
        deposit.interest_earned = 0;
        deposit.last_update = Clock::get()?.unix_timestamp;

        let pool = &mut ctx.accounts.lending_pool;
        pool.total_deposits = pool.total_deposits.checked_add(amount).unwrap_or(u64::MAX);
        Ok(())
    }

    // 脆弱性: 直接invoke_signedとAccountInfo混在
    pub fn vulnerable_liquidation(ctx: Context<VulnerableLiquidation>) -> Result<()> {
        let pool = &mut ctx.accounts.lending_pool;
        
        // 脆弱性: AccountInfoで型安全性欠如
        let borrower_info = &ctx.accounts.borrower_position;
        let liquidator_info = &ctx.accounts.liquidator_position;

        // 脆弱性: 手動データ解析による型回避
        let borrower_data = borrower_info.try_borrow_data()?;
        let liquidator_data = liquidator_info.try_borrow_data()?;

        if borrower_data.len() >= 40 && liquidator_data.len() >= 40 {
            let borrow_amount = u64::from_le_bytes([
                borrower_data[32], borrower_data[33], borrower_data[34], borrower_data[35],
                borrower_data[36], borrower_data[37], borrower_data[38], borrower_data[39]
            ]);

            // 清算処理ループ
            for liquidation_round in 0..7 {
                if borrow_amount > 5000 {
                    let liquidation_amount = (borrow_amount >> liquidation_round) & 0x1FF;
                    pool.total_borrows = pool.total_borrows.saturating_sub(liquidation_amount);
                    
                    // 利息率調整（ビット演算）
                    let rate_adjustment = (liquidation_amount >> 8) & 0x0F;
                    pool.interest_rate = pool.interest_rate.checked_add(rate_adjustment as u32).unwrap_or(50);
                    
                    // 清算ボーナス計算
                    let bonus = liquidation_amount * (liquidation_round + 1) as u64 / 10;
                    pool.total_deposits = pool.total_deposits.checked_add(bonus).unwrap_or(u64::MAX);
                    
                    msg!("Liquidation round {}: amount={}, bonus={}", liquidation_round, liquidation_amount, bonus);
                } else {
                    let penalty = pool.total_deposits / 100;
                    pool.total_deposits = pool.total_deposits.saturating_sub(penalty);
                    
                    // 複利計算
                    let compound_interest = (pool.total_borrows * pool.interest_rate as u64) / 100;
                    pool.total_borrows = pool.total_borrows.checked_add(compound_interest).unwrap_or(u64::MAX);
                    
                    msg!("Penalty applied: {}, compound interest: {}", penalty, compound_interest);
                }
            }
        }

        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitLendingPool<'info> {
    #[account(init, payer = manager, space = 8 + 32 + 8 + 8 + 4)]
    pub lending_pool: Account<'info, LendingPool>,
    #[account(mut)]
    pub manager: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CreateDeposit<'info> {
    #[account(mut)]
    pub lending_pool: Account<'info, LendingPool>,
    #[account(init, payer = depositor, space = 8 + 32 + 32 + 8 + 8 + 8)]
    pub deposit_account: Account<'info, DepositAccount>,
    #[account(mut)]
    pub depositor: Signer<'info>,
    pub system_program: Program<'info, System>,
}

// 脆弱性: AccountInfoによる型検証回避
#[derive(Accounts)]
pub struct VulnerableLiquidation<'info> {
    #[account(mut)]
    pub lending_pool: Account<'info, LendingPool>,
    /// CHECK: 脆弱性 - 借り手ポジション検証なし
    pub borrower_position: AccountInfo<'info>,
    /// CHECK: 脆弱性 - 清算人検証なし
    pub liquidator_position: AccountInfo<'info>,
    pub liquidator: Signer<'info>,
}

#[account]
pub struct LendingPool {
    pub manager: Pubkey,
    pub total_deposits: u64,
    pub total_borrows: u64,
    pub interest_rate: u32,
}

#[account]
pub struct DepositAccount {
    pub pool: Pubkey,
    pub depositor: Pubkey,
    pub amount: u64,
    pub interest_earned: u64,
    pub last_update: i64,
}
