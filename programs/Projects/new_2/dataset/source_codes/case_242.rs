use anchor_lang::prelude::*;

declare_id!("VulnEx41000000000000000000000000000000000041");

#[program]
pub mod example41 {
    pub fn record_chat(ctx: Context<Ctx41>, msg: String) -> Result<()> {
        // chat_room は has_one で moderator 検証済み
        let room = &mut ctx.accounts.chat_room;
        room.messages.push((ctx.accounts.user.key(), msg.clone()));
        // chat_buffer は所有者検証なし
        ctx.accounts.chat_buffer.data.borrow_mut().extend_from_slice(msg.as_bytes());
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ctx41<'info> {
    #[account(mut, has_one = moderator)]
    pub chat_room: Account<'info, ChatRoom>,
    pub moderator: Signer<'info>,
    pub user: Signer<'info>,
    /// CHECK: チャットバッファ、所有者検証なし
    #[account(mut)]
    pub chat_buffer: AccountInfo<'info>,
}

#[account]
pub struct ChatRoom {
    pub moderator: Pubkey,
    pub messages: Vec<(Pubkey, String)>,
}
