use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWxTWq4rove6YjFgDhqyQ5RBwzVL");

#[program]
pub mod document_approval {
    use super::*;

    /// ドキュメント作成：所有者を署名者に設定し、ステータスは `Draft`
    pub fn create_document(
        ctx: Context<CreateDocument>,
        title: String,
        content: String,
    ) -> Result<()> {
        let doc = &mut ctx.accounts.document;
        doc.owner    = ctx.accounts.author.key();
        doc.title    = title;
        doc.content  = content;
        doc.status   = DocStatus::Draft;
        doc.created  = ctx.accounts.clock.unix_timestamp;
        doc.updated  = doc.created;
        Ok(())
    }

    /// ドキュメント承認：所有者のみが呼び出し可能、ステータスを `Approved` に更新
    pub fn approve_document(
        ctx: Context<ModifyDocument>,
    ) -> Result<()> {
        let doc = &mut ctx.accounts.document;
        doc.status  = DocStatus::Approved;
        doc.updated = ctx.accounts.clock.unix_timestamp;
        Ok(())
    }

    /// ドキュメント却下：所有者のみが呼び出し可能、ステータスを `Rejected` に更新
    pub fn reject_document(
        ctx: Context<ModifyDocument>,
    ) -> Result<()> {
        let doc = &mut ctx.accounts.document;
        doc.status  = DocStatus::Rejected;
        doc.updated = ctx.accounts.clock.unix_timestamp;
        Ok(())
    }
}

#[account]
pub struct Document {
    pub owner:   Pubkey,        // ドキュメント所有者
    pub title:   String,        // タイトル
    pub content: String,        // 本文
    pub status:  DocStatus,     // ステータス
    pub created: i64,           // 作成時刻
    pub updated: i64,           // 更新時刻
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum DocStatus {
    Draft,
    Approved,
    Rejected,
}

#[derive(Accounts)]
pub struct CreateDocument<'info> {
    /// ランダムキーで新規作成（PDA ではない）
    #[account(
        init,
        payer = author,
        space = 8                // discriminator
              + 32               // owner
              + 4 + 64           // title (max 64 bytes)
              + 4 + 256          // content (max 256 bytes)
              + 1                // status (enum as u8)
              + 8                // created
              + 8                // updated
    )]
    pub document: Account<'info, Document>,

    /// ドキュメント作成者かつ所有者
    #[account(mut)]
    pub author: Signer<'info>,

    pub clock: Sysvar<'info, Clock>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ModifyDocument<'info> {
    /// 既存の Document（has_one で owner フィールドと一致する signer を要求）
    #[account(
        mut,
        has_one = owner
    )]
    pub document: Account<'info, Document>,

    /// ドキュメント所有者であることを証明
    #[account(signer)]
    pub owner: AccountInfo<'info>,

    pub clock: Sysvar<'info, Clock>,
}
