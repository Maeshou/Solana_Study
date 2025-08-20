use anchor_lang::prelude::*;

declare_id!("SafeEx27ErrorLog1111111111111111111111111111");

#[program]
pub mod example27 {
    use super::*;

    pub fn init_logger(
        ctx: Context<InitLogger>,
        errors: u32,
        warns:  u32,
    ) -> Result<()> {
        let l = &mut ctx.accounts.logger;
        l.error_count   = errors;
        l.warning_count = warns;
        l.critical_flag = false;

        // 重大エラー判定
        if errors > warns * 2 {
            l.critical_flag = true;
        }
        Ok(())
    }

    pub fn log_event(
        ctx: Context<LogEvent>,
        is_error: bool,
    ) -> Result<()> {
        let l = &mut ctx.accounts.logger;
        if is_error {
            l.error_count = l.error_count.saturating_add(1);
        } else {
            l.warning_count = l.warning_count.saturating_add(1);
        }

        // クリティカル再判定
        if l.error_count > l.warning_count * 2 {
            l.critical_flag = true;
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitLogger<'info> {
    #[account(init, payer = user, space = 8 + 4 + 4 + 1)]
    pub logger: Account<'info, ErrorLoggerData>,
    #[account(mut)] pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct LogEvent<'info> {
    #[account(mut)] pub logger: Account<'info, ErrorLoggerData>,
}

#[account]
pub struct ErrorLoggerData {
    pub error_count:   u32,
    pub warning_count: u32,
    pub critical_flag: bool,
}
