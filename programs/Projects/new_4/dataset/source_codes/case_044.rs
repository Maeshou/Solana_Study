// 9. タスクスケジューラ＋実行履歴（Clockなし）
use anchor_lang::prelude::*;
declare_id!("TASKZZZZYYYYXXXXWWWWVVVVUUUUTTTT");

#[program]
pub mod misinit_task_scheduler_no_clock {
    use super::*;

    pub fn init_schedule(ctx: Context<InitSchedule>, cron: String) -> Result<()> {
        let ts = &mut ctx.accounts.schedule;
        ts.cron = cron;
        ts.run_count = 0;
        Ok(())
    }

    pub fn execute_task(ctx: Context<InitSchedule>) -> Result<()> {
        let ts = &mut ctx.accounts.schedule;
        ts.run_count = ts.run_count.checked_add(1).unwrap();
        let log = &mut ctx.accounts.execution_log;
        log.counts.push(ts.run_count);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitSchedule<'info> {
    #[account(init, payer = admin, space = 8 + (4+64) + 1)] pub schedule: Account<'info, ScheduleData>,
    #[account(mut)] pub execution_log: Account<'info, ExecutionLog>,
    #[account(mut)] pub admin: Signer<'info>, pub system_program: Program<'info, System>,
}

#[account]
pub struct ScheduleData { pub cron:String, pub run_count:u8 }
#[account]
pub struct ExecutionLog { pub counts: Vec<u8> }
