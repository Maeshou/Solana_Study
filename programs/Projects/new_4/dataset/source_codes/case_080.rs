use anchor_lang::prelude::*;

declare_id!("NextCaseEx50505050505050505050505050505050");

#[program]
pub mod example5 {
    use super::*;

    // 書籍を貸出（book にだけ init）
    pub fn lend_book(ctx: Context<Lend>, due: i64) -> Result<()> {
        let book = &mut ctx.accounts.book;             // ← initあり
        book.due_date = due;

        let history = &mut ctx.accounts.history;       // ← initなし（本来は初期化すべき）
        // 期限過ぎていれば履歴にフラグ
        if Clock::get()?.unix_timestamp > due {
            history.late = true;
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Lend<'info> {
    #[account(init, payer = librarian, space = 8 + 8)]
    pub book: Account<'info, BookData>,
    pub history: Account<'info, HistoryData>,
    #[account(mut)] pub librarian: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct BookData {
    pub due_date: i64,
}

#[account]
pub struct HistoryData {
    pub late: bool,
}
