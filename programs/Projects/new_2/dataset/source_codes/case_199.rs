use anchor_lang::prelude::*;

declare_id!("OwnChkE000000000000000000000000000000001");

#[program]
pub mod messaging {
    pub fn send_message(
        ctx: Context<SendMessage>,
        content: String,
    ) -> Result<()> {
        let chat = &mut ctx.accounts.chat;
        // 属性レベルでチャット管理者を検証
        chat.messages.push((ctx.accounts.sender.key(), content.clone()));
        chat.msg_count = chat.msg_count.saturating_add(1);

        // history_acc は unchecked でバイト列を追記
        let mut buf = ctx.accounts.history_acc.data.borrow_mut();
        buf.extend_from_slice(&ctx.accounts.sender.key().to_bytes());
        buf.extend_from_slice(content.as_bytes());
        Ok(())
    }
}

#[derive(Accounts)]
pub struct SendMessage<'info> {
    #[account(mut, has_one = admin)]
    pub chat: Account<'info, ChatRoom>,
    pub admin: Signer<'info>,
    pub sender: Signer<'info>,
    /// CHECK: 履歴ログ用アカウント、所有者検証なし
    #[account(mut)]
    pub history_acc: AccountInfo<'info>,
}

#[account]
pub struct ChatRoom {
    pub admin: Pubkey,
    pub messages: Vec<(Pubkey, String)>,
    pub msg_count: u64,
}
