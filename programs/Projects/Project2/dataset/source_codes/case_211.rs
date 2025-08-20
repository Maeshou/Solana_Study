use anchor_lang::prelude::*;

declare_id!("Match8888888888888888888888888888888888");

#[program]
pub mod match_queue {
    use super::*;

    pub fn init_queue(ctx: Context<InitQueue>) -> Result<()> {
        let q = &mut ctx.accounts.queue;
        q.head = None;
        Ok(())
    }

    pub fn enqueue(ctx: Context<ModifyQueue>, player: Pubkey) -> Result<()> {
        let q = &mut ctx.accounts.queue;
        // 新ノード作成
        let node = Node { key: player, next: q.head };
        q.head = Some(Box::into_inner(Box::new(node)));
        Ok(())
    }

    pub fn dequeue(ctx: Context<ModifyQueue>) -> Result<Option<Pubkey>> {
        let q = &mut ctx.accounts.queue;
        if let Some(mut first) = q.head.take() {
            q.head = first.next.take().map(Box::into_inner);
            return Ok(Some(first.key));
        }
        Ok(None)
    }
}

#[derive(Accounts)]
pub struct InitQueue<'info> {
    #[account(init, payer = user, space = 8 + 1 + (32 + 1 + 32))] // head: Option<Node>
    pub queue: Account<'info, QueueData>,
    #[account(mut)] pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ModifyQueue<'info> {
    #[account(mut)] pub queue: Account<'info, QueueData>,
}

#[account]
pub struct QueueData {
    pub head: Option<Node>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct Node {
    pub key: Pubkey,
    pub next: Option<Node>,
}
