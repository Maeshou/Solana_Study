use anchor_lang::prelude::*;

declare_id!("VulnVarX3000000000000000000000000000000003");

#[program]
pub mod example3 {
    pub fn rotate_queue(ctx: Context<Ctx3>, steps: u8) -> Result<()> {
        // aux_buf は unchecked
        let mut aux = ctx.accounts.aux_buf.data.borrow_mut();
        aux.rotate_left((steps as usize).min(aux.len()));

        // task_queue は has_one 検証済み
        let queue = &mut ctx.accounts.task_queue.items;
        queue.rotate_right((steps as usize) % queue.len());
        ctx.accounts.task_queue.rotate_count = ctx.accounts.task_queue.rotate_count.saturating_add(1);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ctx3<'info> {
    /// CHECK: 補助バッファ、所有者検証なし
    #[account(mut)]
    pub aux_buf: AccountInfo<'info>,

    #[account(mut, has_one = manager)]
    pub task_queue: Account<'info, TaskQueue>,
    pub manager: Signer<'info>,
}

#[account]
pub struct TaskQueue {
    pub manager: Pubkey,
    pub items: Vec<u8>,
    pub rotate_count: u64,
}
