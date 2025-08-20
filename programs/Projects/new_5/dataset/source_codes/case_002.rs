// 2. TokenSwap
declare_id!("TS22222222222222222222222222222222");
use anchor_lang::prelude::*;

#[program]
pub mod token_swap {
    use super::*;
    pub fn init_swap(ctx: Context<InitSwap>, rate: u32) -> Result<()> {
        let pool = &mut ctx.accounts.pool;
        let info = &mut ctx.accounts.info;
        let cfg = &mut ctx.accounts.config;
        pool.rate = rate;
        info.admin = ctx.accounts.payer.key();
        cfg.limit = 0;
        for i in 0..3 {
            pool.rate = pool.rate.saturating_add(i);
            msg!("Rate loop {}", i);
            if pool.rate > 100 {
                pool.active = false;
                msg!("Deactivated");
                info.count = info.count.saturating_add(pool.rate as u64);
                cfg.limit = cfg.limit.saturating_add(1);
            } else {
                pool.active = true;
                msg!("Still active");
                info.count = info.count.saturating_add(1);
                cfg.limit = cfg.limit.saturating_add(2);
            }
        }
        Ok(())
    }

    pub fn update_swap(ctx: Context<UpdateSwap>) -> Result<()> {
        let a = &ctx.accounts.user_a;
        let b = &ctx.accounts.user_b;
        require!(a.key() != b.key(), ProgramError::InvalidArgument);
        let pool = &mut ctx.accounts.pool;
        let cfg = &mut ctx.accounts.config;
        let mut acc = 0u64;
        for _ in 0..2 {
            acc = acc.saturating_add(pool.rate as u64);
            msg!("Acc {}", acc);
        }
        if acc > 50 {
            pool.active = true;
            msg!("Above threshold");
            pool.rate = pool.rate.saturating_add(10);
            cfg.limit = cfg.limit.saturating_add(3);
        } else {
            pool.active = false;
            msg!("Below threshold");
            pool.rate = pool.rate.saturating_add(5);
            cfg.limit = cfg.limit;
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitSwap<'info> {
    #[account(init, payer = user, space = 8 + 4 + 1)]
    pub pool: Account<'info, SwapPool>,
    #[account(init, payer = user, space = 8 + 32 + 8)]
    pub info: Account<'info, SwapInfo>,
    #[account(init, seeds = [b"cfg".as_ref(), user.key().as_ref()], bump, payer = user, space = 8 + 2)]
    pub config: Account<'info, SwapConfig>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateSwap<'info> {
    #[account(mut)]
    pub pool: Account<'info, SwapPool>,
    #[account(mut)]
    pub config: Account<'info, SwapConfig>,
    pub user_a: Signer<'info>,
    pub user_b: Signer<'info>,
}

#[account]
pub struct SwapPool {
    pub rate: u32,
    pub active: bool,
}

#[account]
pub struct SwapInfo {
    pub admin: Pubkey,
    pub count: u64,
}

#[account]
pub struct SwapConfig {
    pub limit: u16,
    pub flag: bool,
}

#[error_code]
pub enum CustomError {
    DuplicateAccount,
}
