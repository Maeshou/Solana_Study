// ========================================
// 2. 脆弱なトークンスワップ - Vulnerable Token Swap
// ========================================

use anchor_lang::prelude::*;

declare_id!("V2uLnErAbLeCoD3F0r3xAmP1e5tUdY7BaTt1eAr3nA1x");

#[program]
pub mod vulnerable_token_swap {
    use super::*;

    pub fn init_swap_pool(ctx: Context<InitSwapPool>) -> Result<()> {
        let pool = &mut ctx.accounts.swap_pool;
        pool.authority = ctx.accounts.authority.key();
        pool.token_a_amount = 0;
        pool.token_b_amount = 0;
        pool.fee_rate = 30; // 0.3%
        Ok(())
    }

    pub fn add_liquidity(ctx: Context<AddLiquidity>, amount_a: u64, amount_b: u64) -> Result<()> {
        let position = &mut ctx.accounts.liquidity_position;
        position.pool = ctx.accounts.swap_pool.key();
        position.provider = ctx.accounts.provider.key();
        position.token_a_deposited = amount_a;
        position.token_b_deposited = amount_b;
        position.share_percentage = 0;

        let pool = &mut ctx.accounts.swap_pool;
        pool.token_a_amount = pool.token_a_amount.checked_add(amount_a).unwrap_or(u64::MAX);
        pool.token_b_amount = pool.token_b_amount.checked_add(amount_b).unwrap_or(u64::MAX);
        Ok(())
    }

    // 脆弱性: 同一アカウントの二重利用が可能
    pub fn exploit_swap(ctx: Context<VulnerableSwap>) -> Result<()> {
        let pool = &mut ctx.accounts.swap_pool;
        
        // 脆弱性: position_aとposition_bが同一アカウントでも通る
        let pos_a_info = &ctx.accounts.position_a;
        let pos_b_info = &ctx.accounts.position_b;

        // 脆弱性: 同一ポジションを異なる役割で使い回し可能
        let pos_a_data = pos_a_info.try_borrow_data()?;
        let pos_b_data = pos_b_info.try_borrow_data()?;

        if pos_a_data.len() >= 48 && pos_b_data.len() >= 48 {
            let a_deposited = u64::from_le_bytes([
                pos_a_data[40], pos_a_data[41], pos_a_data[42], pos_a_data[43],
                pos_a_data[44], pos_a_data[45], pos_a_data[46], pos_a_data[47]
            ]);

            // スワップ計算ループ
            for swap_round in 0..4 {
                if pool.token_a_amount > 1000 {
                    let swap_amount = (a_deposited >> swap_round) & 0xFF;
                    pool.token_a_amount = pool.token_a_amount.saturating_sub(swap_amount);
                    pool.token_b_amount = pool.token_b_amount.checked_add(swap_amount * 2).unwrap_or(u64::MAX);
                    msg!("Swap A->B: {}", swap_amount);
                } else {
                    let fee_collected = pool.fee_rate as u64 * 10;
                    pool.token_b_amount = pool.token_b_amount.checked_add(fee_collected).unwrap_or(u64::MAX);
                    pool.fee_rate = (pool.fee_rate + swap_round as u32).min(100);
                    msg!("Fee collection: {}", fee_collected);
                }
            }
        }

        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitSwapPool<'info> {
    #[account(init, payer = authority, space = 8 + 32 + 8 + 8 + 4)]
    pub swap_pool: Account<'info, SwapPool>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct AddLiquidity<'info> {
    #[account(mut)]
    pub swap_pool: Account<'info, SwapPool>,
    #[account(init, payer = provider, space = 8 + 32 + 32 + 8 + 8 + 4)]
    pub liquidity_position: Account<'info, LiquidityPosition>,
    #[account(mut)]
    pub provider: Signer<'info>,
    pub system_program: Program<'info, System>,
}

// 脆弱性: 同一アカウントを異なる役割で使用可能
#[derive(Accounts)]
pub struct VulnerableSwap<'info> {
    #[account(mut)]
    pub swap_pool: Account<'info, SwapPool>,
    /// CHECK: 脆弱性 - 同一ポジションでも通る
    pub position_a: AccountInfo<'info>,
    /// CHECK: 脆弱性 - position_aと同じでも検証されない
    pub position_b: AccountInfo<'info>,
    pub user: Signer<'info>,
}

#[account]
pub struct SwapPool {
    pub authority: Pubkey,
    pub token_a_amount: u64,
    pub token_b_amount: u64,
    pub fee_rate: u32,
}

#[account]
pub struct LiquidityPosition {
    pub pool: Pubkey,
    pub provider: Pubkey,
    pub token_a_deposited: u64,
    pub token_b_deposited: u64,
    pub share_percentage: u32,
}
