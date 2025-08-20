use anchor_lang::prelude::*;

declare_id!("SafeEx31ThreadPool111111111111111111111111");

#[program]
pub mod example31 {
    use super::*;

    pub fn init_pool(
        ctx: Context<InitPool>,
        threads: u8,
    ) -> Result<()> {
        let p = &mut ctx.accounts.pool;
        p.threads        = threads;
        p.active_threads = 0;
        p.overload_flag  = false;

        // 初期稼働率判定
        if threads > 4 {
            p.overload_flag = true;
        }
        Ok(())
    }

    pub fn spawn_thread(
        ctx: Context<SpawnThread>,
    ) -> Result<()> {
        let p = &mut ctx.accounts.pool;
        if p.active_threads < p.threads {
            p.active_threads = p.active_threads.saturating_add(1);
        } else {
            p.overload_flag = true;
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitPool<'info> {
    #[account(init, payer = user, space = 8 + 1 + 1 + 1)]
    pub pool: Account<'info, ThreadPoolData>,
    #[account(mut)] pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct SpawnThread<'info> {
    #[account(mut)] pub pool: Account<'info, ThreadPoolData>,
}

#[account]
pub struct ThreadPoolData {
    pub threads:        u8,
    pub active_threads: u8,
    pub overload_flag:  bool,
}
