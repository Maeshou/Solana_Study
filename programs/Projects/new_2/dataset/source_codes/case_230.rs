use anchor_lang::prelude::*;

declare_id!("VulnEx28000000000000000000000000000000000028");

#[program]
pub mod message_service {
    pub fn enqueue(ctx: Context<Ctx8>, msg: String) -> Result<()> {
        // message_acc は未検証
        ctx.accounts.message_acc.data.borrow_mut().extend_from_slice(msg.as_bytes());
        // queue は has_one で owner 検証済み
        let q = &mut ctx.accounts.queue;
        q.messages.push(msg);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ctx8<'info> {
    #[account(mut, has_one = owner)]
    pub queue: Account<'info, MessageQueue>,
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
    #[account(mut)]
    pub message_acc: AccountInfo<'info>,
}
