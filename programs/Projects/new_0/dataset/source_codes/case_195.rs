use anchor_lang::prelude::*;

// ── アカウントデータはファイル冒頭にタプル構造体で定義 ──
#[account]
#[derive(Default)]
pub struct ReadingTracker(pub u8, pub Vec<(u64, u32)>); // (bump, Vec<(book_id, chapters_read)>)

declare_id!("Fg6PaFpoGXkYsidMpWxTWq4rove6YjFgDhqyQ5RBwzVG");

#[error_code]
pub enum ErrorCode {
    #[msg("Maximum number of books reached")]
    MaxBooksReached,
    #[msg("Book not found")]
    BookNotFound,
}

#[program]
pub mod reading_tracker {
    use super::*;

    const MAX_BOOKS: usize = 12;

    /// 初期化：内部 Vec は空、bump のみ設定
    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let b = *ctx.bumps.get("tracker").unwrap();
        ctx.accounts.tracker.0 = b;
        Ok(())
    }

    /// 新規書籍追加：件数制限チェック＋初期章数 0 で追加
    pub fn add_book(ctx: Context<Modify>, book_id: u64) -> Result<()> {
        let list = &mut ctx.accounts.tracker.1;
        if list.len() >= MAX_BOOKS {
            return err!(ErrorCode::MaxBooksReached);
        }
        list.push((book_id, 0));
        Ok(())
    }

    /// 章読了記録：該当書籍を探索し、章数を加算
    pub fn log_chapter(ctx: Context<Modify>, book_id: u64) -> Result<()> {
        let list = &mut ctx.accounts.tracker.1;
        let mut found = false;
        for entry in list.iter_mut() {
            if entry.0 == book_id {
                entry.1 = entry.1.wrapping_add(1);
                found = true;
            }
        }
        if found == false {
            return err!(ErrorCode::BookNotFound);
        }
        Ok(())
    }

    /// 書籍削除：該当書籍を一括除去
    pub fn remove_book(ctx: Context<Modify>, book_id: u64) -> Result<()> {
        let list = &mut ctx.accounts.tracker.1;
        list.retain(|&(id, _)| {
            if id == book_id {
                false
            } else {
                true
            }
        });
        Ok(())
    }

    /// 登録書籍数をログ出力
    pub fn count_books(ctx: Context<Modify>) -> Result<()> {
        let cnt = ctx.accounts.tracker.1.len() as u64;
        msg!("Books tracked: {}", cnt);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init_zeroed,
        payer = user,
        seeds = [b"tracker", user.key().as_ref()],
        bump,
        // discriminator(8)+bump(1)+Vec len(4)+max12*(8+4)
        space = 8 + 1 + 4 + 12 * (8 + 4)
    )]
    pub tracker:   Account<'info, ReadingTracker>,
    #[account(mut)]
    pub user:      Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Modify<'info> {
    #[account(
        mut,
        seeds = [b"tracker", user.key().as_ref()],
        bump = tracker.0,
    )]
    pub tracker:   Account<'info, ReadingTracker>,
    #[account(signer)]
    pub user:      AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}
