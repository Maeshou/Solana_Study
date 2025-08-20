use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgMailSvc001");

#[program]
pub mod mail_service {
    use super::*;

    /// 新しいメッセージをボックスに送信するが、
    /// mailbox_account.owner と ctx.accounts.user.key() の一致検証がない
    pub fn send_mail(
        ctx: Context<ModifyMailbox>,
        message_id: u64,
        content: String,
    ) -> Result<()> {
        let mailbox = &mut ctx.accounts.mailbox;
        add_message(mailbox, message_id, content);
        Ok(())
    }

    /// 指定のメッセージをボックスから削除するが、
    /// mailbox_account.owner と ctx.accounts.user.key() の一致検証がない
    pub fn delete_mail(
        ctx: Context<ModifyMailbox>,
        message_id: u64,
    ) -> Result<()> {
        let mailbox = &mut ctx.accounts.mailbox;
        remove_message(mailbox, message_id);
        Ok(())
    }
}

/// メッセージを追加し、カウンタと未読リストを更新するヘルパー
fn add_message(mailbox: &mut MailboxAccount, message_id: u64, content: String) {
    mailbox.message_ids.push(message_id);
    mailbox.contents.push(content);
    mailbox.unread_count = mailbox.unread_count.checked_add(1).unwrap();
    mailbox.total_received = mailbox.total_received.checked_add(1).unwrap();
}

/// メッセージを削除し、カウンタと未読リストを更新するヘルパー
fn remove_message(mailbox: &mut MailboxAccount, message_id: u64) {
    if let Some(pos) = mailbox.message_ids.iter().position(|&id| id == message_id) {
        mailbox.message_ids.remove(pos);
        mailbox.contents.remove(pos);
        mailbox.delete_count = mailbox.delete_count.checked_add(1).unwrap();
        // 未読だった場合は未読カウンタをデクリメント
        if mailbox.unread_ids.iter().any(|&id| id == message_id) {
            mailbox.unread_count = mailbox.unread_count.checked_sub(1).unwrap();
            mailbox.unread_ids.retain(|&id| id != message_id);
        }
    }
}

#[derive(Accounts)]
pub struct ModifyMailbox<'info> {
    #[account(mut)]
    /// 本来は `#[account(has_one = owner)]` を指定して所有者照合を行うべき
    pub mailbox: Account<'info, MailboxAccount>,
    /// 操作をリクエストするユーザー（署名者）
    pub user: Signer<'info>,
}

#[account]
pub struct MailboxAccount {
    /// 本来このボックスを所有するべきユーザーの Pubkey
    pub owner: Pubkey,
    /// 受信したメッセージの ID 一覧
    pub message_ids: Vec<u64>,
    /// 各メッセージの本文
    pub contents: Vec<String>,
    /// 未読メッセージの ID 一覧
    pub unread_ids: Vec<u64>,
    /// 未読メッセージ数
    pub unread_count: u64,
    /// これまでに受信したメッセージの総数
    pub total_received: u64,
    /// これまでに削除したメッセージの総数
    pub delete_count: u64,
}
