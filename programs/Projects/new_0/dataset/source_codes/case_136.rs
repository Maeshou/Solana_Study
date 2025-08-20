use anchor_lang::prelude::*;

declare_id!("BkrL111111111111111111111111111111111111");

const MAX_BORROWERS: usize = 100;

#[program]
pub mod library_manager {
    /// 新しい書籍を登録
    pub fn add_book(
        ctx: Context<AddBook>,
        title: String,
        copies: u32,
    ) -> Result<()> {
        // タイトル長チェック
        if title.len() > 64 {
            return Err(ErrorCode::TitleTooLong.into());
        }
        // 冊数は1以上
        if copies == 0 {
            return Err(ErrorCode::InvalidCopies.into());
        }

        let book = &mut ctx.accounts.book;
        book.owner     = ctx.accounts.user.key(); // Signer Authorization
        book.title     = title;
        book.available = copies;
        book.borrowers = Vec::new();
        Ok(())
    }

    /// 書籍を借りる
    pub fn borrow_book(ctx: Context<ModifyBook>) -> Result<()> {
        let book = &mut ctx.accounts.book;
        let user = ctx.accounts.user.key();

        // 所有者は借りられない
        if book.owner == user {
            return Err(ErrorCode::CannotBorrowOwn.into());
        }
        // 在庫があるか
        if book.available == 0 {
            return Err(ErrorCode::NoCopiesLeft.into());
        }
        // 既に借りていないかチェック
        for &b in book.borrowers.iter() {
            if b == user {
                return Err(ErrorCode::AlreadyBorrowed.into());
            }
        }
        // 真ブランチで複数処理：在庫減算＋借り手追加
        book.available = book
            .available
            .checked_sub(1)
            .ok_or(ErrorCode::Underflow)?;
        book.borrowers.push(user);
        Ok(())
    }

    /// 書籍を返却する
    pub fn return_book(ctx: Context<ModifyBook>) -> Result<()> {
        let book = &mut ctx.accounts.book;
        let user = ctx.accounts.user.key();
        let mut idx: Option<usize> = None;

        // 所有者は返却不要
        if book.owner == user {
            return Err(ErrorCode::CannotReturnOwn.into());
        }
        // 借り手リスト探索
        for (i, &b) in book.borrowers.iter().enumerate() {
            if b == user {
                idx = Some(i);
                break;
            }
        }
        // 見つかったら複数処理：リスト削除＋在庫加算
        if let Some(i) = idx {
            book.borrowers.remove(i);
            book.available = book
                .available
                .checked_add(1)
                .ok_or(ErrorCode::Overflow)?;
            Ok(())
        } else {
            Err(ErrorCode::NotBorrowed.into())
        }
    }
}

#[derive(Accounts)]
pub struct AddBook<'info> {
    /// Reinit Attack 防止
    #[account(init, payer = user, space = 8 + 32 + 4 + 64 + 4 + 4 + (MAX_BORROWERS * 32))]
    pub book:           Account<'info, BookAccount>,

    /// 登録者（署名必須）
    #[account(mut)]
    pub user:           Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ModifyBook<'info> {
    /// 型チェック＆Owner Check
    #[account(mut)]
    pub book:           Account<'info, BookAccount>,

    /// 借り手／返却者（署名必須）
    pub user:           Signer<'info>,
}

#[account]
pub struct BookAccount {
    /// 管理者としての所有者
    pub owner:     Pubkey,
    /// 書籍タイトル（最大64文字）
    pub title:     String,
    /// 利用可能な在庫数
    pub available: u32,
    /// 借り手リスト（最大100名）
    pub borrowers: Vec<Pubkey>,
}

#[error_code]
pub enum ErrorCode {
    #[msg("権限がありません")]
    Unauthorized,
    #[msg("タイトルが長すぎます")]
    TitleTooLong,
    #[msg("冊数は1以上である必要があります")]
    InvalidCopies,
    #[msg("自分の本は借りられません")]
    CannotBorrowOwn,
    #[msg("在庫がありません")]
    NoCopiesLeft,
    #[msg("既に借りています")]
    AlreadyBorrowed,
    #[msg("借りていません")]
    NotBorrowed,
    #[msg("オーバーフローしました")]
    Overflow,
    #[msg("アンダーフローしました")]
    Underflow,
}
