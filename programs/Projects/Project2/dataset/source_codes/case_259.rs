use anchor_lang::prelude::*;

declare_id!("MatchQ0888888888888888888888888888888888");

#[program]
pub mod match_queue {
    use super::*;

    pub fn enqueue(ctx: Context<Enq>, player: Pubkey) -> Result<()> {
        let q = &mut ctx.accounts.queue;
        let next = (q.tail + 1) % q.capacity;
        if next != q.head {
            q.buffer[q.tail as usize] = Some(player);
            q.tail = next;
            q.length = q.length.saturating_add(1);
        }
        Ok(())
    }

    pub fn dequeue(ctx: Context<Enq>) -> Result<Option<Pubkey>> {
        let q = &mut ctx.accounts.queue;
        if q.length > 0 {
            let res = q.buffer[q.head as usize].take();
            q.head = (q.head + 1) % q.capacity;
            q.length = q.length.saturating_sub(1);
            Ok(res)
        } else {
            Ok(None)
        }
    }
}

#[derive(Accounts)]
pub struct Enq<'info> {
    #[account(mut)]
    pub queue: Account<'info, MatchQueueData>,
}

#[account]
pub struct MatchQueueData {
    pub buffer: [Option<Pubkey>; 16],
    pub head: u8,
    pub tail: u8,
    pub length: u8,
    pub capacity: u8, // =16
}
