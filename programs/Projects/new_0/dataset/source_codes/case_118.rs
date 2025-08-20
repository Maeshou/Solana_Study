use anchor_lang::prelude::*;

declare_id!("Read111111111111111111111111111111111111");

#[program]
pub mod reading_list {
    /// 新しい本をリストに追加
    pub fn add_book(
        ctx: Context<AddBook>,
        title: String,
        author: String,
    ) -> Result<()> {
        // 入力長チェック
        if title.len() > 64 {
            return Err(ErrorCode::TitleTooLong.into());
        }
        if author.len() > 64 {
            return Err(ErrorCode::AuthorTooLong.into());
        }

        let book = &mut ctx.accounts.book;
        book.owner  = ctx.accounts.user.key();
        book.title  = title;
        book.author = author;
        book.read   = false;
        Ok(())
    }

    /// 本の情報を更新（タイトル・著者・既読フラグ変更）
    pub fn update_book(
        ctx: Context<UpdateBook>,
        new_title: String,
        new_author: String,
        read: bool,
    ) -> Result<()> {
        // 入力長チェック
        if new_title.len() > 64 {
            return Err(ErrorCode::TitleTooLong.into());
        }
        if new_author.len() > 64 {
            return Err(ErrorCode::AuthorTooLong.into());
        }

        let book = &mut ctx.accounts.book;
        // 所有者チェック
        if book.owner != ctx.accounts.user.key() {
            return Err(ErrorCode::Unauthorized.into());
        }

        book.title  = new_title;
        book.author = new_author;
        book.read   = read;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct AddBook<'info> {
    /// 同一アカウントを二度初期化できない（Reinit Attack 防止）
    #[account(init, payer = user, space = 8 + 32 + 4 + 64 + 4 + 64 + 1)]
    pub book:           Account<'info, BookAccount>,

    /// 操作を行うユーザー（署名必須）
    #[account(mut)]
    pub user:           Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateBook<'info> {
    /// Anchor の Account<> による Owner Check & Type Cosplay
    #[account(mut)]
    pub book:           Account<'info, BookAccount>,

    /// 実際に署名したユーザー
    pub user:           Signer<'info>,
}

#[account]
pub struct BookAccount {
    /// この本情報を操作できるユーザー
    pub owner:  Pubkey,
    /// 本のタイトル（最大64文字）
    pub title:  String,
    /// 著者名（最大64文字）
    pub author: String,
    /// 既読済みかどうか
    pub read:   bool,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Unauthorized")]
    Unauthorized,
    #[msg("Title is too long")]
    TitleTooLong,
    #[msg("Author name is too long")]
    AuthorTooLong,
}
