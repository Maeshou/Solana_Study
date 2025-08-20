use anchor_lang::prelude::*;
declare_id!("ChatGrpVuln1111111111111111111111111111111");

/// チャットグループ情報
#[account]
pub struct ChatGroup {
    pub owner:     Pubkey,       // グループ管理者
    pub title:     String,       // グループ名
    pub message_count: u64,      // 送信済みメッセージ数
}

/// メッセージ記録
#[account]
pub struct MessageRecord {
    pub sender:    Pubkey,       // 送信者
    pub group:     Pubkey,       // 本来は ChatGroup.key() と一致すべき
    pub content:   String,       // メッセージ内容
}

#[derive(Accounts)]
pub struct CreateGroup<'info> {
    #[account(init, payer = owner, space = 8 + 32 + 4 + 64 + 8)]
    pub group:     Account<'info, ChatGroup>,
    #[account(mut)]
    pub owner:     Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct SendMessage<'info> {
    /// MessageRecord.sender == sender.key() は検証される
    #[account(mut, has_one = sender)]
    pub record:    Account<'info, MessageRecord>,

    /// ChatGroup.owner == owner.key() は不要だが、
    /// MessageRecord.group ⇔ group.key() の検証がないまま参照
    #[account(mut)]
    pub group:     Account<'info, ChatGroup>,

    #[account(mut)]
    pub sender:    Signer<'info>,
}

#[derive(Accounts)]
pub struct PostMessage<'info> {
    /// ChatGroup.owner == owner.key() は検証される
    #[account(mut, has_one = owner)]
    pub group:     Account<'info, ChatGroup>,

    /// MessageRecord.group ⇔ group.key() の検証がない
    #[account(mut)]
    pub record:    Account<'info, MessageRecord>,

    pub owner:     Signer<'info>,
}

#[program]
pub mod chatgroup_vuln {
    use super::*;

    /// グループを作成
    pub fn create_group(ctx: Context<CreateGroup>, title: String) -> Result<()> {
        let g = &mut ctx.accounts.group;
        g.owner         = ctx.accounts.owner.key();
        g.title         = title;
        g.message_count = 0;
        Ok(())
    }

    /// メッセージを送信（記録に内容を書き込む）
    pub fn send_message(ctx: Context<SendMessage>, text: String) -> Result<()> {
        let rec = &mut ctx.accounts.record;
        let grp = &mut ctx.accounts.group;

        // 脆弱性ポイント：
        // rec.group = grp.key(); の検証なしでメッセージを上書き
        rec.sender  = ctx.accounts.sender.key();
        rec.group   = grp.key();
        rec.content = text.clone();

        // message_count を直接加算
        grp.message_count = grp.message_count + 1;
        Ok(())
    }

    /// グループ所有者がメッセージを更新
    pub fn post_message(ctx: Context<PostMessage>, new_text: String) -> Result<()> {
        let rec = &mut ctx.accounts.record;
        let grp = &ctx.accounts.group;

        // 本来は必須：
        // require_keys_eq!(rec.group, grp.key(), ErrorCode::GroupMismatch);

        // メッセージ文字列を置き換え
        rec.content = new_text;
        Ok(())
    }
}

#[error_code]
pub enum ErrorCode {
    #[msg("MessageRecord が指定の ChatGroup と一致しません")]
    GroupMismatch,
}
