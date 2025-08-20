use anchor_lang::prelude::*;
declare_id!("ReadListVuln1111111111111111111111111111111");

/// ユーザーの読書リスト
#[account]
pub struct ReadingList {
    pub owner:  Pubkey,         // リスト所有者
    pub books:  Vec<Pubkey>,    // 本のID一覧
}

/// 本の追加・削除を記録するアカウント
#[account]
pub struct ReadingRecord {
    pub user:     Pubkey,       // 操作したユーザー
    pub list:     Pubkey,       // 本来は ReadingList.key() と一致すべき
    pub action:   String,       // "added" か "removed"
    pub book_id:  Pubkey,       // 対象の本ID
}

#[derive(Accounts)]
pub struct CreateList<'info> {
    #[account(init, payer = owner, space = 8 + 32 + 4 + 32 * 10)]
    pub list:      Account<'info, ReadingList>,
    #[account(mut)]
    pub owner:     Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct AddBook<'info> {
    /// ReadingList.owner == owner.key() は検証される
    #[account(mut, has_one = owner)]
    pub list:      Account<'info, ReadingList>,

    /// ReadingRecord.list ⇔ list.key() の検証がないため、
    /// 任意の Recording を渡しても通ってしまう
    #[account(init, payer = owner, space = 8 + 32 + 32 + 4 + 32 + 4 + 64)]
    pub record:    Account<'info, ReadingRecord>,

    #[account(mut)]
    pub owner:     Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct RemoveBook<'info> {
    /// ReadingList.owner == owner.key() は検証される
    #[account(mut, has_one = owner)]
    pub list:      Account<'info, ReadingList>,

    /// ReadingRecord.list と list.key() の一致チェックがない
    #[account(mut)]
    pub record:    Account<'info, ReadingRecord>,

    pub owner:     Signer<'info>,
}

#[program]
pub mod reading_list_vuln {
    use super::*;

    pub fn create_list(ctx: Context<CreateList>) -> Result<()> {
        let rl = &mut ctx.accounts.list;
        rl.owner = ctx.accounts.owner.key();
        // Vec フィールドは初期化時に空ベクターとなるため、特に代入不要
        Ok(())
    }

    pub fn add_book(ctx: Context<AddBook>, book_id: Pubkey) -> Result<()> {
        let rl = &mut ctx.accounts.list;
        let rec = &mut ctx.accounts.record;

        // 脆弱性ポイント：
        // rec.list = rl.key(); と代入するだけで、
        // ReadingRecord.list と ReadingList.key() の検証がない
        rec.user    = ctx.accounts.owner.key();
        rec.list    = rl.key();
        rec.action  = String::from("added");
        rec.book_id = book_id;

        // Vec に本を追加
        rl.books.push(book_id);

        Ok(())
    }

    pub fn remove_book(ctx: Context<RemoveBook>, book_id: Pubkey) -> Result<()> {
        let rl = &mut ctx.accounts.list;
        let rec = &mut ctx.accounts.record;

        // 本来は必須：
        // require_keys_eq!(rec.list, rl.key(), ErrorCode::ListMismatch);

        // Vec から対象の本IDを取り除く（filter を使った新規ベクタ再構築）
        rl.books = rl
            .books
            .iter()
            .copied()
            .filter(|id| id != &book_id)
            .collect();

        rec.user    = ctx.accounts.owner.key();
        rec.action  = String::from("removed");
        rec.book_id = book_id;

        Ok(())
    }
}

#[error_code]
pub enum ErrorCode {
    #[msg("ReadingRecord が指定の ReadingList と一致しません")]
    ListMismatch,
}
