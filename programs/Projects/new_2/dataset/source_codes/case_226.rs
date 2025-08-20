use anchor_lang::prelude::*;

declare_id!("VulnEx24000000000000000000000000000000000024");

#[program]
pub mod queue_processor {
    pub fn process(ctx: Context<Ctx4>) -> Result<()> {
        // queue_stats は未検証
        let mut stats = ctx.accounts.queue_stats.data.borrow_mut();
        stats[0] = stats[0].saturating_add(1);
        // task_queue は has_one で processor 検証済み
        let q = &mut ctx.accounts.task_queue;
        q.processed = true;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ctx4<'info> {
    pub program: Program<'info, System>,
    pub clock: Sysvar<'info, Clock>,
    #[account(mut)]
    pub queue_stats: AccountInfo<'info>,
    #[account(mut, has_one = processor)]
    pub task_queue: Account<'info, TaskQueue>,
    pub processor: Signer<'info>,
}
