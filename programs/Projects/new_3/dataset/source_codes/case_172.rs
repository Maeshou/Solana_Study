use anchor_lang::prelude::*;
declare_id!("DocTagVuln1111111111111111111111111111111");

/// ドキュメント情報
#[account]
pub struct Document {
    pub owner: Pubkey,           // ドキュメント所有者
    pub title: String,           // タイトル
    pub tags: Vec<String>,       // タグ一覧
}

/// タグ操作記録
#[account]
pub struct TagRecord {
    pub user:     Pubkey,        // 操作者
    pub document: Pubkey,        // 本来は Document.key() と一致すべき
    pub tag:      String,        // 操作したタグ
}

#[derive(Accounts)]
pub struct CreateDocument<'info> {
    #[account(init, payer = owner, space = 8 + 32 + 4 + 128 + 4 + (32 * 10))]
    pub document: Account<'info, Document>,
    #[account(mut)]
    pub owner:    Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct AddTag<'info> {
    /// Document.owner == owner.key() は検証される
    #[account(mut, has_one = owner)]
    pub document: Account<'info, Document>,

    /// TagRecord.document ⇔ document.key() の検証がない
    #[account(init, payer = owner, space = 8 + 32 + 32 + 4 + 32)]
    pub record:   Account<'info, TagRecord>,

    #[account(mut)]
    pub owner:    Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ClearTags<'info> {
    /// TagRecord.user == user.key() は検証される
    #[account(mut, has_one = user)]
    pub record:   Account<'info, TagRecord>,

    /// Document.key() と record.document の検証がない
    #[account(mut)]
    pub document: Account<'info, Document>,

    pub user:     Signer<'info>,
}

#[program]
pub mod doc_tag_vuln {
    use super::*;

    /// 新しいドキュメントを作成
    pub fn create_document(ctx: Context<CreateDocument>, title: String) -> Result<()> {
        let doc = &mut ctx.accounts.document;
        doc.owner = ctx.accounts.owner.key();
        doc.title = title;
        // tags は init 時に空ベクタになるので追加代入不要
        Ok(())
    }

    /// タグを追加
    pub fn add_tag(ctx: Context<AddTag>, tag: String) -> Result<()> {
        let doc = &mut ctx.accounts.document;
        let rec = &mut ctx.accounts.record;

        // 脆弱性ポイント：
        // rec.document = doc.key(); と設定しているだけで、
        // TagRecord.document と Document.key() の一致検証が一切ない
        rec.user     = ctx.accounts.owner.key();
        rec.document = doc.key();
        rec.tag      = tag.clone();

        // Vec::push でタグを追加
        doc.tags.push(tag);
        Ok(())
    }

    /// すべてのタグをクリア（タグ一覧を空にする）
    pub fn clear_tags(ctx: Context<ClearTags>) -> Result<()> {
        let doc = &mut ctx.accounts.document;
        // 本来は必須：
        // require_keys_eq!(ctx.accounts.record.document, doc.key(), ErrorCode::Mismatch);

        // Vec::clear で全タグを一括削除
        doc.tags.clear();
        Ok(())
    }
}

#[error_code]
pub enum ErrorCode {
    #[msg("TagRecord が指定の Document と一致しません")]
    Mismatch,
}
