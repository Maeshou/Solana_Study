use anchor_lang::prelude::*;

declare_id!("SafeEx17RateLimit11111111111111111111111111");

#[program]
pub mod example17 {
    use super::*;

    pub fn init_rate_limiter(
        ctx: Context<InitRateLimiter>,
        limit: u32,
    ) -> Result<()> {
        let rl = &mut ctx.accounts.rate_limiter;
        rl.limit = limit;
        rl.used  = 0;
        rl.reset_counter = 0;

        // 初期状態ならリセットカウンタを段階的に
        let mut cnt = 0u32;
        while cnt < limit {
            cnt += 1;
        }
        rl.reset_counter = cnt;
        Ok(())
    }

    pub fn use_slot(
        ctx: Context<UseSlot>,
    ) -> Result<()> {
        let rl = &mut ctx.accounts.rate_limiter;
        // 空きがあるかチェック
        if rl.used < rl.limit {
            rl.used += 1;
        } else {
            // オーバー時はリセット
            rl.used = 0;
            rl.reset_counter = rl.limit;
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitRateLimiter<'info> {
    #[account(init, payer = user, space = 8 + 4 + 4 + 4)]
    pub rate_limiter: Account<'info, RateLimiterData>,
    #[account(mut)] pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UseSlot<'info> {
    #[account(mut)] pub rate_limiter: Account<'info, RateLimiterData>,
}

#[account]
pub struct RateLimiterData {
    pub limit:         u32,
    pub used:          u32,
    pub reset_counter: u32,
}
