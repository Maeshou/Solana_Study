use anchor_lang::prelude::*;

declare_id!("UserNoteNoClk22222222222222222222222222222");

#[program]
pub mod personal_note_noclk {
    use super::*;

    /// メモを新規作成または上書きし、保存バージョンを更新
    pub fn save_note(ctx: Context<SaveNote>, text: String) -> Result<()> {
        let note = &mut ctx.accounts.note_acc;
        // 本文を上書き
        note.content = text.clone();
        // バージョンをインクリメント
        note.version = note.version.checked_add(1).unwrap();
        // 履歴に追加
        note.history.push(format!("Saved as v{}", note.version));
        Ok(())
    }

    /// 任意のエントリを履歴に追加
    pub fn add_entry(ctx: Context<AppendLog>, entry: String) -> Result<()> {
        let note = &mut ctx.accounts.note_acc;
        note.history.push(entry);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct SaveNote<'info> {
    /// ユーザーごとに PDA を導出
    #[account(
        init_if_needed,
        seeds = [b"note", user.key().as_ref()],
        bump,
        payer = user,
        space = 
            8 +                  // discriminator
            4 + 200 +            // content: String（最大200バイト）
            8 +                  // version: u64
            4 + (4 + 100) * 10   // history: Vec<String>（最大10件×100バイト）
    )]
    pub note_acc: Account<'info, NoteData>,

    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct AppendLog<'info> {
    /// 既存の PDA にのみアクセス
    #[account(
        mut,
        seeds = [b"note", user.key().as_ref()],
        bump
    )]
    pub note_acc: Account<'info, NoteData>,

    pub user: Signer<'info>,
}

#[account]
pub struct NoteData {
    pub content: String,
    pub version: u64,
    pub history: Vec<String>,
}
