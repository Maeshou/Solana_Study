use anchor_lang::prelude::*;

declare_id!("SafeEx26Uptime11111111111111111111111111111");

#[program]
pub mod example26 {
    use super::*;

    pub fn init_uptime(
        ctx: Context<InitUptime>,
        cycles: u32,
    ) -> Result<()> {
        let u = &mut ctx.accounts.uptime;
        u.start_count = cycles;
        u.failures    = 0;
        u.stable_flag = false;

        // 安定稼働判定
        if cycles > 1000 {
            u.stable_flag = true;
        }
        Ok(())
    }

    pub fn record_failure(
        ctx: Context<RecordFailure>,
    ) -> Result<()> {
        let u = &mut ctx.accounts.uptime;
        u.failures = u.failures.saturating_add(1);

        // 安定フラグクリア
        if u.failures > u.start_count / 10 {
            u.stable_flag = false;
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitUptime<'info> {
    #[account(init, payer = user, space = 8 + 4 + 4 + 1)]
    pub uptime: Account<'info, UptimeData>,
    #[account(mut)] pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct RecordFailure<'info> {
    #[account(mut)] pub uptime: Account<'info, UptimeData>,
}

#[account]
pub struct UptimeData {
    pub start_count: u32,
    pub failures:    u32,
    pub stable_flag: bool,
}
