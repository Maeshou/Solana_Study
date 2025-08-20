use anchor_lang::prelude::*;
use anchor_lang::solana_program::sysvar::clock::Clock;

// ドキュメント承認ワークフロー管理プログラム
declare_id!("Appr111111111111111111111111111111111111");

#[program]
pub mod approval_manager {
    /// ドキュメントを提出
    pub fn submit_document(
        ctx: Context<SubmitDocument>,
        title: String,
        content: String,
    ) -> Result<()> {
        let doc = &mut ctx.accounts.document;
        let now = ctx.accounts.clock.unix_timestamp;

        // 入力検証
        require!(title.len() <= 64, ErrorCode::TitleTooLong);
        require!(content.len() <= 1024, ErrorCode::ContentTooLong);
        require!(
            ctx.accounts.requester.key() != ctx.accounts.approver.key(),
            ErrorCode::SameApprover
        );

        // ドキュメント初期化
        doc.owner = ctx.accounts.requester.key();
        doc.approver = ctx.accounts.approver.key();
        doc.title = title;
        doc.content = content;
        doc.status = Status::Pending as u8;
        doc.submitted_at = now;
        doc.reviewed_at = 0;
        Ok(())
    }

    /// ドキュメントを承認
    pub fn approve_document(
        ctx: Context<ReviewDocument>
    ) -> Result<()> {
        let doc = &mut ctx.accounts.document;
        let now = ctx.accounts.clock.unix_timestamp;

        // 承認者チェック
        require!(doc.approver == ctx.accounts.approver.key(), ErrorCode::Unauthorized);
        // ステータスが Pending であること
        require!(doc.status == Status::Pending as u8, ErrorCode::InvalidStatus);

        // 承認処理
        doc.status = Status::Approved as u8;
        doc.reviewed_at = now;
        Ok(())
    }

    /// ドキュメントを却下
    pub fn reject_document(
        ctx: Context<ReviewDocument>
    ) -> Result<()> {
        let doc = &mut ctx.accounts.document;
        let now = ctx.accounts.clock.unix_timestamp;

        // 承認者チェック
        require!(doc.approver == ctx.accounts.approver.key(), ErrorCode::Unauthorized);
        // ステータスが Pending
        require!(doc.status == Status::Pending as u8, ErrorCode::InvalidStatus);

        // 却下処理
        doc.status = Status::Rejected as u8;
        doc.reviewed_at = now;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct SubmitDocument<'info> {
    #[account(init, payer = requester, space = 8 + 32 + 32 + 4 + 64 + 4 + 1024 + 1 + 8 + 8)]
    pub document: Account<'info, DocumentAccount>,
    /// 提出者
    #[account(mut)] pub requester: Signer<'info>,
    /// 承認者として指定されるアカウント
    pub approver: AccountInfo<'info>,
    pub clock: Sysvar<'info, Clock>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ReviewDocument<'info> {
    #[account(mut)] pub document: Account<'info, DocumentAccount>,
    /// 実際の承認者
    pub approver: Signer<'info>,
    pub clock: Sysvar<'info, Clock>,
}

#[account]
pub struct DocumentAccount {
    pub owner:         Pubkey,
    pub approver:      Pubkey,
    pub title:         String,
    pub content:       String,
    pub status:        u8,
    pub submitted_at:  i64,
    pub reviewed_at:   i64,
}

#[error_code]
pub enum ErrorCode {
    #[msg("アクセスが許可されていません")] Unauthorized,
    #[msg("タイトルが長すぎます")] TitleTooLong,
    #[msg("本文が長すぎます")] ContentTooLong,
    #[msg("承認者と提出者は同一にできません")] SameApprover,
    #[msg("ステータスが不正です")] InvalidStatus,
}

/// ドキュメントのステータス
pub enum Status {
    Pending = 0,
    Approved = 1,
    Rejected = 2,
}
