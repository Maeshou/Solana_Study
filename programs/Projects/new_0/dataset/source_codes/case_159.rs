use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWxTWq4rove6YjFgDhqyQ5RBwzUE");

#[program]
pub mod library_manager {
    use super::*;

    /// 本の追加：PDA でアカウント作成し、タイトル・著者・貸出状況・最終貸出者をまとめて初期化
    pub fn add_book(
        ctx: Context<AddBook>,
        bump: u8,
        book_id: u64,
        title: String,
        author: String,
    ) -> Result<()> {
        *ctx.accounts.book = Book {
            owner:        ctx.accounts.librarian.key(),
            bump,
            book_id,
            title,
            author,
            available:    true,
            last_reader:  ctx.accounts.librarian.key(),
        };
        Ok(())
    }

    /// 本の貸出：available を false にし、last_reader を申請者に更新
    pub fn check_out(
        ctx: Context<ModifyBook>,
    ) -> Result<()> {
        let book = &mut ctx.accounts.book;
        book.available   = false;
        book.last_reader = ctx.accounts.user.key();
        Ok(())
    }

    /// 本の返却：available を true に戻す
    pub fn check_in(
        ctx: Context<ModifyBook>,
    ) -> Result<()> {
        ctx.accounts.book.available = true;
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(bump: u8, book_id: u64)]
pub struct AddBook<'info> {
    /// PDA で生成する Book アカウント
    #[account(
        init,
        payer = librarian,
        // discriminator(8) + owner Pubkey(32) + bump(1) + book_id(8)
        // + title 長さプレフィクス(4)+最大100バイト + author 同(4+100)
        // + available(1) + last_reader Pubkey(32)
        space = 8 + 32 + 1 + 8 + 4 + 100 + 4 + 100 + 1 + 32,
        seeds = [b"book", librarian.key().as_ref(), &book_id.to_le_bytes()],
        bump
    )]
    pub book: Account<'info, Book>,

    /// 図書館管理者（署名必須）
    #[account(mut)]
    pub librarian: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ModifyBook<'info> {
    /// 既存の Book（PDA＋bump 検証のみ、誰でも貸出・返却可能）
    #[account(
        mut,
        seeds = [b"book", book.owner.as_ref(), &book.book_id.to_le_bytes()],
        bump = book.bump,
    )]
    pub book: Account<'info, Book>,

    /// 処理を実行するユーザー（署名必須）
    #[account(signer)]
    pub user: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
}

/// Book データ構造：所有者・bump・ID・タイトル・著者・貸出可否・最終貸出者を保持
#[account]
pub struct Book {
    pub owner:       Pubkey,
    pub bump:        u8,
    pub book_id:     u64,
    pub title:       String,
    pub author:      String,
    pub available:   bool,
    pub last_reader: Pubkey,
}
