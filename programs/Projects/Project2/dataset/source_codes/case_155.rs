use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWxTWq4rove6YjFgDhqyQ5RBwzUA");

#[program]
pub mod bookmark_manager {
    use super::*;

    /// ブックマーク作成：URL とタイトルを受け取り、PDA で一意のアカウントを生成
    pub fn create_bookmark(
        ctx: Context<CreateBookmark>,
        bump: u8,
        bookmark_id: u64,
        title: String,
        url: String,
    ) -> Result<()> {
        let bm = &mut ctx.accounts.bookmark;
        bm.owner       = ctx.accounts.user.key();
        bm.bump        = bump;
        bm.bookmark_id = bookmark_id;
        bm.title       = title;
        bm.url         = url;
        Ok(())
    }

    /// ブックマーク更新：タイトルと URL を書き換え
    pub fn update_bookmark(
        ctx: Context<UpdateBookmark>,
        new_title: String,
        new_url: String,
    ) -> Result<()> {
        let bm = &mut ctx.accounts.bookmark;
        bm.title = new_title;
        bm.url   = new_url;
        Ok(())
    }

    /// ブックマーク削除：close 属性でアカウント解放＆残高返却
    pub fn delete_bookmark(ctx: Context<DeleteBookmark>) -> Result<()> {
        // 閉鎖は属性に任せるため、関数内処理は不要
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(bump: u8, bookmark_id: u64)]
pub struct CreateBookmark<'info> {
    /// PDA で生成する Bookmark アカウント
    #[account(
        init,
        payer = user,
        // discriminator(8) + owner Pubkey(32) + bump(1) + bookmark_id(8)
        // + String len prefix(4) + タイトル最大100バイト + URL最大200バイト
        space = 8 + 32 + 1 + 8 + 4 + 100 + 4 + 200,
        seeds = [b"bookmark", user.key().as_ref(), &bookmark_id.to_le_bytes()],
        bump
    )]
    pub bookmark: Account<'info, Bookmark>,

    /// ブックマーク所有者（署名必須）
    #[account(mut)]
    pub user: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(bookmark_id: u64)]
pub struct UpdateBookmark<'info> {
    /// 既存の Bookmark（PDA／bump 検証 + オーナーチェック）
    #[account(
        mut,
        seeds = [b"bookmark", owner.key().as_ref(), &bookmark_id.to_le_bytes()],
        bump = bookmark.bump,
        has_one = owner
    )]
    pub bookmark: Account<'info, Bookmark>,

    /// Bookmark 所有者（署名必須）
    #[account(signer)]
    pub owner: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(bookmark_id: u64)]
pub struct DeleteBookmark<'info> {
    /// 閉鎖対象の Bookmark（PDA／bump 検証 + オーナーチェック + close）
    #[account(
        mut,
        seeds = [b"bookmark", owner.key().as_ref(), &bookmark_id.to_le_bytes()],
        bump = bookmark.bump,
        has_one = owner,
        close = owner
    )]
    pub bookmark: Account<'info, Bookmark>,

    /// Bookmark 所有者（署名必須）
    #[account(signer)]
    pub owner: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
}

/// Bookmark データ構造：所有者、bump、一意ID、タイトル、URL を保持
#[account]
pub struct Bookmark {
    pub owner: Pubkey,
    pub bump: u8,
    pub bookmark_id: u64,
    pub title: String,
    pub url: String,
}
