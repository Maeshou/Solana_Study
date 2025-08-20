use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgMsgPinSvc001");

#[program]
pub mod message_pin_service {
    use super::*;

    /// メッセージをピン留めするが、
    /// message_account.owner と ctx.accounts.user.key() の照合チェックがない
    pub fn pin_message(ctx: Context<PinMessage>) -> Result<()> {
        let msg = &mut ctx.accounts.message_account;
        mark_as_pinned(msg);
        Ok(())
    }

    /// メッセージのピン留めを解除するが、
    /// message_account.owner と ctx.accounts.user.key() の照合チェックがない
    pub fn unpin_message(ctx: Context<UnpinMessage>) -> Result<()> {
        let msg = &mut ctx.accounts.message_account;
        clear_pinned(msg);
        Ok(())
    }
}

/// メッセージをピン状態にし、カウンタをインクリメントするヘルパー関数
fn mark_as_pinned(msg: &mut Message) {
    msg.pinned = true;
    msg.pin_count = msg.pin_count.checked_add(1).unwrap();
}

/// メッセージのピンを解除し、アンピンカウンタをインクリメントするヘルパー関数
fn clear_pinned(msg: &mut Message) {
    msg.pinned = false;
    msg.unpin_count = msg.unpin_count.checked_add(1).unwrap();
}

#[derive(Accounts)]
pub struct PinMessage<'info> {
    #[account(mut)]
    /// 本来は #[account(has_one = owner)] を指定して所有者一致を検証すべき
    pub message_account: Account<'info, Message>,
    /// ピン操作をリクエストするユーザー（署名者）
    pub user: Signer<'info>,
}

#[derive(Accounts)]
pub struct UnpinMessage<'info> {
    #[account(mut)]
    /// 本来は #[account(has_one = owner)] を指定して所有者一致を検証すべき
    pub message_account: Account<'info, Message>,
    /// アンピン操作をリクエストするユーザー（署名者）
    pub user: Signer<'info>,
}

#[account]
pub struct Message {
    /// 本来このメッセージを作成したユーザーの Pubkey
    pub owner: Pubkey,
    /// ピン留め中かどうか
    pub pinned: bool,
    /// ピン留めされた回数
    pub pin_count: u64,
    /// アンピンされた回数
    pub unpin_count: u64,
    /// メッセージ本文
    pub content: String,
}
