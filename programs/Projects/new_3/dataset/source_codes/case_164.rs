use anchor_lang::prelude::*;
declare_id!("LibLoan1111111111111111111111111111111111");

/// 図書管理アカウント
#[account]
pub struct Book {
    pub librarian:    Pubkey, // 貸出管理者
    pub total_loans:  u64,    // 累計貸出回数
}

/// 貸出記録
#[account]
pub struct LoanRecord {
    pub borrower:     Pubkey, // 本を借りた利用者
    pub book:         Pubkey, // 本来は Book.key() と一致すべき
    pub due_timestamp: i64,   // 返却期限（UNIXタイム）
}

#[derive(Accounts)]
pub struct AddBook<'info> {
    #[account(init, payer = librarian, space = 8 + 32 + 8)]
    pub book:         Account<'info, Book>,
    #[account(mut)]
    pub librarian:    Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct LoanBook<'info> {
    /// Book.librarian == librarian.key() は検証される
    #[account(mut, has_one = librarian)]
    pub book:         Account<'info, Book>,

    /// LoanRecord.book ⇔ Book.key() の照合がないまま初期化
    #[account(init, payer = borrower, space = 8 + 32 + 32 + 8)]
    pub record:       Account<'info, LoanRecord>,

    #[account(mut)]
    pub borrower:     Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ReturnBook<'info> {
    /// LoanRecord.borrower == borrower.key() は検証される
    #[account(mut, has_one = borrower)]
    pub record:       Account<'info, LoanRecord>,

    /// Book.librarian == librarian.key() は検証される
    #[account(mut, has_one = librarian)]
    pub book:         Account<'info, Book>,

    /// LoanRecord.book ⇔ Book.key() の一致検証がないため、
    /// 別の LoanRecord を渡されても通ってしまう
    pub librarian:    Signer<'info>,
}

#[program]
pub mod library_vuln {
    use super::*;

    /// 新しい本を登録
    pub fn add_book(ctx: Context<AddBook>) -> Result<()> {
        let b = &mut ctx.accounts.book;
        b.librarian   = ctx.accounts.librarian.key();
        b.total_loans = 0;
        Ok(())
    }

    /// 本を貸し出し
    pub fn loan_book(ctx: Context<LoanBook>, due_ts: i64) -> Result<()> {
        let b = &mut ctx.accounts.book;
        let r = &mut ctx.accounts.record;

        // 脆弱性ポイント：
        // r.book = b.key(); と設定するだけで、
        // LoanRecord.book と Book.key() の照合を行っていない
        r.borrower      = ctx.accounts.borrower.key();
        r.book          = b.key();
        r.due_timestamp = due_ts;

        // 累計貸出回数を更新（checked_add＋unwrap_or で扱う）
        b.total_loans = b
            .total_loans
            .checked_add(1)
            .unwrap_or(b.total_loans);
        Ok(())
    }

    /// 本を返却
    pub fn return_book(ctx: Context<ReturnBook>) -> Result<()> {
        let b = &mut ctx.accounts.book;
        // 本来は必須：
        // require_keys_eq!(ctx.accounts.record.book, b.key(), ErrorCode::RecordMismatch);
        //
        // このチェックがないため、攻撃者は任意の LoanRecord を渡して
        // b.total_loans を不正に調整できる

        // 返却時には貸出記録を消去するかわりに回数を減算
        b.total_loans = b
            .total_loans
            .checked_sub(1)
            .unwrap_or(b.total_loans);
        Ok(())
    }
}

#[error_code]
pub enum ErrorCode {
    #[msg("LoanRecord が指定の Book と一致しません")]
    RecordMismatch,
}
