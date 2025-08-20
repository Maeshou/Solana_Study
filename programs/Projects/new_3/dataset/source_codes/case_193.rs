use anchor_lang::prelude::*;
declare_id!("BookmarkVuln11111111111111111111111111111");

/// ブックマークリスト情報
#[account]
pub struct BookmarkList {
    pub owner:    Pubkey,       // リスト所有者
    pub name:     String,       // リスト名
    pub items:    Vec<String>,  // ブックマークURL一覧
}

/// ブックマーク記録
#[account]
pub struct BookmarkRecord {
    pub user:     Pubkey,       // 記録を作成したユーザー
    pub list:     Pubkey,       // 本来は BookmarkList.key() と一致すべき
    pub url:      String,       // ブックマークURL
}

#[derive(Accounts)]
pub struct CreateList<'info> {
    #[account(init, payer = owner, space = 8 + 32 + 4 + 64 + 4 + (4 + 128) * 100)]
    pub list:     Account<'info, BookmarkList>,
    #[account(mut)]
    pub owner:    Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct AddBookmark<'info> {
    /// BookmarkList.owner == owner.key() は検証される
    #[account(mut, has_one = owner)]
    pub list:     Account<'info, BookmarkList>,

    /// BookmarkRecord.list ⇔ list.key() の検証がないため、
    /// 偽のレコードで任意のリストに追加できてしまう
    #[account(init, payer = user, space = 8 + 32 + 32 + 4 + 256)]
    pub record:   Account<'info, BookmarkRecord>,

    #[account(mut)]
    pub owner:    Signer<'info>,
    #[account(mut)]
    pub user:     Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ClearBookmarks<'info> {
    /// BookmarkRecord.user == user.key() は検証される
    #[account(mut, has_one = user)]
    pub record:   Account<'info, BookmarkRecord>,

    /// list.key() ⇔ record.list の検証がないため、
    /// 偽物のレコードで別リストをクリアできる
    #[account(mut)]
    pub list:     Account<'info, BookmarkList>,

    pub user:     Signer<'info>,
}

#[program]
pub mod bookmark_vuln {
    use super::*;

    /// ブックマークリストを作成
    pub fn create_list(ctx: Context<CreateList>, name: String) -> Result<()> {
        let lst = &mut ctx.accounts.list;
        lst.owner = ctx.accounts.owner.key();
        lst.name  = name;
        // items は init 時に空 Vec
        Ok(())
    }

    /// ブックマークURLを追加
    pub fn add_bookmark(ctx: Context<AddBookmark>, url: String) -> Result<()> {
        let lst = &mut ctx.accounts.list;
        let rec = &mut ctx.accounts.record;

        // 脆弱性ポイント:
        // rec.list = lst.key(); の一致検証がない
        rec.user = ctx.accounts.user.key();
        rec.list = lst.key();
        rec.url  = url.clone();

        // Vec::push で URL を追加
        lst.items.push(url);
        Ok(())
    }

    /// 全ブックマークをクリア
    pub fn clear_bookmarks(ctx: Context<ClearBookmarks>) -> Result<()> {
        let lst = &mut ctx.accounts.list;

        // 本来必要:
        // require_keys_eq!(ctx.accounts.record.list, lst.key(), ErrorCode::Mismatch);

        // Vec::clear で全 URL を削除（分岐・ループなし）
        lst.items.clear();
        Ok(())
    }
}
