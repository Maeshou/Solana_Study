use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWxTWq4rove6YjFgDhqyQ5RBwzT9");

#[program]
pub mod message_board {
    use super::*;

    /// メッセージ投稿：user＋message_id で PDA を生成し、内容を保存
    pub fn create_message(
        ctx: Context<CreateMessage>,
        bump: u8,
        message_id: u64,
        content: String,
    ) -> Result<()> {
        let message = &mut ctx.accounts.message;
        message.owner      = ctx.accounts.user.key();
        message.bump       = bump;
        message.message_id = message_id;
        message.content    = content;
        Ok(())
    }

    /// メッセージ更新：has_one + signer で所有者のみ更新可能
    pub fn update_message(
        ctx: Context<UpdateMessage>,
        new_content: String,
    ) -> Result<()> {
        let message = &mut ctx.accounts.message;
        message.content = new_content;
        Ok(())
    }

    /// メッセージ削除：close 属性でアカウントを解放し、残高を owner に返却
    pub fn delete_message(ctx: Context<DeleteMessage>) -> Result<()> {
        // 本体は空。close＝owner によってアカウントが閉じられる
        Ok(())
    }
}

/// 投稿用アカウント定義
#[derive(Accounts)]
#[instruction(bump: u8, message_id: u64)]
pub struct CreateMessage<'info> {
    /// PDA で生成する Message アカウント
    #[account(
        init,
        payer = user,
        // discriminator(8) + owner Pubkey(32) + bump(1) + message_id(8) + String の長さプレフィクス(4) + content 最大140バイト
        space = 8 + 32 + 1 + 8 + 4 + 140,
        seeds = [b"message", user.key().as_ref(), &message_id.to_le_bytes()],
        bump
    )]
    pub message: Account<'info, Message>,

    /// トランザクション送信者（投稿者）
    #[account(mut)]
    pub user: Signer<'info>,

    pub system_program: Program<'info, System>,
}

/// 更新用アカウント定義
#[derive(Accounts)]
#[instruction(message_id: u64)]
pub struct UpdateMessage<'info> {
    /// 既存の Message（PDA／bump 検証 + オーナーチェック）
    #[account(
        mut,
        seeds = [b"message", owner.key().as_ref(), &message_id.to_le_bytes()],
        bump = message.bump,
        has_one = owner
    )]
    pub message: Account<'info, Message>,

    /// Message 所有者（署名必須）
    #[account(signer)]
    pub owner: AccountInfo<'info>,
}

/// 削除用アカウント定義
#[derive(Accounts)]
#[instruction(message_id: u64)]
pub struct DeleteMessage<'info> {
    /// 閉鎖対象の Message（PDA／bump 検証 + オーナーチェック + close）
    #[account(
        mut,
        seeds = [b"message", owner.key().as_ref(), &message_id.to_le_bytes()],
        bump = message.bump,
        has_one = owner,
        close = owner
    )]
    pub message: Account<'info, Message>,

    /// Message 所有者（署名必須）
    #[account(signer)]
    pub owner: AccountInfo<'info>,
}

/// Message データ構造：所有者、bump、ID、内容を保持
#[account]
pub struct Message {
    pub owner: Pubkey,
    pub bump: u8,
    pub message_id: u64,
    pub content: String,
}
