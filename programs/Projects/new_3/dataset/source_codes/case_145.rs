use anchor_lang::prelude::*;
declare_id!("DocApprv1111111111111111111111111111111111");

/// ドキュメント情報
#[account]
pub struct Document {
    pub owner:            Pubkey, // 作成者
    pub title:            String, // タイトル
    pub content_hash:     [u8; 32], // 本文のハッシュ
    pub approvals_count:  u64,    // 承認数
}

/// 承認記録
#[account]
pub struct ApprovalRecord {
    pub approver:   Pubkey,   // 承認者
    pub document:   Pubkey,   // 本来は Document.key() と一致すべき
    pub approved:   bool,     // 承認済みフラグ
}

#[derive(Accounts)]
pub struct CreateDocument<'info> {
    #[account(init, payer = creator, space = 8 + 32 + 4 + 64 + 32 + 8)]
    pub document:       Account<'info, Document>,
    #[account(mut)]
    pub creator:        Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct RequestApproval<'info> {
    #[account(mut, has_one = owner)]
    pub document:       Account<'info, Document>,
    #[account(init, payer = requester, space = 8 + 32 + 32 + 1)]
    pub approval_record: Account<'info, ApprovalRecord>,
    #[account(mut)]
    pub requester:      Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ApproveDocument<'info> {
    /// Document.owner == owner.key() は検証される
    #[account(mut, has_one = owner)]
    pub document:       Account<'info, Document>,

    /// ApprovalRecord.document == document.key() の検証がない
    #[account(mut)]
    pub approval_record: Account<'info, ApprovalRecord>,

    pub owner:          Signer<'info>,
}

#[program]
pub mod document_vuln {
    use super::*;

    /// ドキュメントを作成
    pub fn create_document(
        ctx: Context<CreateDocument>,
        title: String,
        content_hash: [u8; 32],
    ) -> Result<()> {
        let doc = &mut ctx.accounts.document;
        doc.owner           = ctx.accounts.creator.key();
        doc.title           = title;
        doc.content_hash    = content_hash;
        doc.approvals_count = 0;
        msg!("Document '{}' created by {}", doc.title, doc.owner);
        Ok(())
    }

    /// 承認要求を初期化
    pub fn request_approval(ctx: Context<RequestApproval>) -> Result<()> {
        let rec = &mut ctx.accounts.approval_record;
        rec.approver = ctx.accounts.requester.key();
        rec.document = ctx.accounts.document.key();
        rec.approved = false;
        msg!("Approval requested by {} for document {}", rec.approver, rec.document);
        Ok(())
    }

    /// ドキュメントを承認
    pub fn approve(ctx: Context<ApproveDocument>) -> Result<()> {
        let doc = &mut ctx.accounts.document;
        let rec = &mut ctx.accounts.approval_record;

        // 本来は以下のような検証が必要：
        // require_keys_eq!(
        //     rec.document,
        //     doc.key(),
        //     DocError::RecordMismatch
        // );
        //
        // もしくは
        // #[account(address = document.key())]
        // pub approval_record: Account<'info, ApprovalRecord>;

        // 検証がないため、攻撃者は任意の ApprovalRecord を渡して
        // approvals_count を好きに増加できる
        rec.approved = true;
        doc.approvals_count = doc.approvals_count.checked_add(1).unwrap();
        msg!(
            "Document {} approved by {}, total approvals: {}",
            doc.key(),
            ctx.accounts.owner.key(),
            doc.approvals_count
        );
        Ok(())
    }
}

#[error_code]
pub enum DocError {
    #[msg("ApprovalRecord が指定の Document と一致しません")]
    RecordMismatch,
}
