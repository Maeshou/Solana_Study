use anchor_lang::prelude::*;

declare_id!("CraftQ660606060606060606060606060606060");

#[program]
pub mod craft_queue {
    use super::*;

    pub fn enqueue(ctx: Context<Enq>, recipe: u64) -> Result<()> {
        let q = &mut ctx.accounts.queue;
        if (q.tail + 1) % (q.slots.len() as u8) != q.head {
            q.slots[q.tail as usize] = Some(recipe);
            q.tail = (q.tail + 1) % (q.slots.len() as u8);
            q.len = q.len.saturating_add(1);
        }
        Ok(())
    }

    pub fn dequeue(ctx: Context<Enq>) -> Result<Option<u64>> {
        let q = &mut ctx.accounts.queue;
        if q.len > 0 {
            let val = q.slots[q.head as usize].take();
            q.head = (q.head + 1) % (q.slots.len() as u8);
            q.len = q.len.saturating_sub(1);
            Ok(val)
        } else {
            Ok(None)
        }
    }
}

#[derive(Accounts)]
pub struct Enq<'info> {
    #[account(mut)]
    pub queue: Account<'info, CraftQueueData>,
}

#[account]
pub struct CraftQueueData {
    pub slots: [Option<u64>; 10],
    pub head: u8,
    pub tail: u8,
    pub len: u8,
}
