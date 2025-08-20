use anchor_lang::prelude::*;

declare_id!("Repertory15Pool111111111111111111111111111111");

#[program]
pub mod liquidity {
    use super::*;

    // プールを初期化
    pub fn init_pool(ctx: Context<InitPool>, fee_bps: u16) -> Result<()> {
        let p = &mut ctx.accounts.pool;
        p.token_a_reserve = 0;
        p.token_b_reserve = 0;
        p.fee_bps = fee_bps;
        Ok(())
    }

    // スワップを実行
    pub fn swap(ctx: Context<Swap>, amount_in: u64) -> Result<()> {
        let p = &mut ctx.accounts.pool;          // ← initなし：既存参照
        // 単純比例計算（手数料除く）
        let amount_after_fee = amount_in - (amount_in * p.fee_bps as u64 / 10_000);
        if p.token_a_reserve > 0 {
            let amount_out = amount_after_fee * p.token_b_reserve / p.token_a_reserve;
            p.token_a_reserve += amount_after_fee;
            p.token_b_reserve -= amount_out;
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitPool<'info> {
    #[account(init, payer = user, space = 8 + 8*2 + 2)]
    pub pool: Account<'info, PoolData>,
    #[account(mut)] pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Swap<'info> {
    pub pool: Account<'info, PoolData>,
    #[account(mut)] pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct PoolData {
    pub token_a_reserve: u64,
    pub token_b_reserve: u64,
    pub fee_bps: u16,
}
