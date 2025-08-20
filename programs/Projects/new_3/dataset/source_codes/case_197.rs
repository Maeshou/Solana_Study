use anchor_lang::prelude::*;
declare_id!("DocSignVuln111111111111111111111111111111");

/// 文書情報
#[account]
pub struct DocumentAccount {
    pub owner:    Pubkey,        // 文書作成者
    pub title:    String,        // 文書タイトル
    pub signers:  Vec<Pubkey>,   // 署名したユーザー一覧
}

/// 署名記録
#[account]
pub struct SignatureRecord {
    pub signer:       Pubkey,    // 署名ユーザー
    pub document_key: Pubkey,    // 本来は DocumentAccount.key() と一致すべき
    pub comment:      String,    // 署名コメント
}

#[derive(Accounts)]
pub struct CreateDocument<'info> {
    #[account(init, payer = creator, space = 8 + 32 + 4 + 128 + 4 + (32 * 50))]
    pub document_account: Account<'info, DocumentAccount>,
    #[account(mut)]
    pub creator:          Signer<'info>,
    pub system_program:   Program<'info, System>,
}

#[derive(Accounts)]
pub struct SignDocument<'info> {
    /// DocumentAccount.owner == creator.key() は検証される
    #[account(mut, has_one = creator)]
    pub document_account: Account<'info, DocumentAccount>,

    /// SignatureRecord.document_key ⇔ document_account.key() の検証がないため、
    /// 偽の SignatureRecord を渡して任意の文書に署名できる
    #[account(init, payer = signer, space = 8 + 32 + 32 + 4 + 256)]
    pub signature_record_account: Account<'info, SignatureRecord>,

    #[account(mut)]
    pub creator:          Signer<'info>,
    #[account(mut)]
    pub signer:           Signer<'info>,
    pub system_program:   Program<'info, System>,
}

#[derive(Accounts)]
pub struct UndoSignature<'info> {
    /// SignatureRecord.signer == signer.key() は検証される
    #[account(mut, has_one = signer)]
    pub signature_record_account: Account<'info, SignatureRecord>,

    /// document_account.key() ⇔ signature_record_account.document_key の検証がないため、
    /// 偽のレコードで別の文書の署名を取り消せる
    #[account(mut)]
    pub document_account: Account<'info, DocumentAccount>,

    pub signer:           Signer<'info>,
}

#[program]
pub mod document_signing_vuln {
    use super::*;

    pub fn create_document(ctx: Context<CreateDocument>, title: String) -> Result<()> {
        let doc = &mut ctx.accounts.document_account;
        doc.owner   = ctx.accounts.creator.key();
        doc.title   = title;
        // signers は初期化時に空 Vec
        Ok(())
    }

    pub fn sign_document(ctx: Context<SignDocument>, comment: String) -> Result<()> {
        let doc = &mut ctx.accounts.document_account;
        let sig = &mut ctx.accounts.signature_record_account;

        // 脆弱性ポイント:
        // sig.document_key = doc.key(); の一致検証がない
        sig.signer       = ctx.accounts.signer.key();
        sig.document_key = doc.key();
        sig.comment      = comment.clone();

        // Vec::push で署名ユーザーを追加
        doc.signers.push(sig.signer);
        Ok(())
    }

    pub fn undo_signature(ctx: Context<UndoSignature>) -> Result<()> {
        let doc = &mut ctx.accounts.document_account;

        // 本来必要:
        // require_keys_eq!(ctx.accounts.signature_record_account.document_key, doc.key(), ErrorCode::Mismatch);

        // Vec::truncate で最後に追加された署名ユーザーを除去（分岐・ループなし）
        let remaining = doc.signers.len().saturating_sub(1);
        doc.signers.truncate(remaining);
        Ok(())
    }
}
