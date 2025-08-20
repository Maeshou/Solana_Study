use anchor_lang::prelude::*;
declare_id!("AnnSysVuln111111111111111111111111111111");

/// お知らせ情報
#[account]
pub struct Announcement {
    pub author:        Pubkey,       // お知らせ作成者
    pub message:       String,       // 本文
    pub acknowledgers: Vec<Pubkey>,  // 既読ユーザー一覧
}

/// 既読記録
#[account]
pub struct AckRecord {
    pub user:       Pubkey,          // 既読を行ったユーザー
    pub announcement: Pubkey,        // 本来は Announcement.key() と一致すべき
    pub note:       String,          // 任意メモ
}

#[derive(Accounts)]
pub struct CreateAnnouncement<'info> {
    #[account(init, payer = author, space = 8 + 32 + 4 + 256 + 4 + (32 * 100))]
    pub announcement: Account<'info, Announcement>,
    #[account(mut)]
    pub author:       Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct MarkRead<'info> {
    /// Announcement.author == author.key() は検証される
    #[account(mut, has_one = author)]
    pub announcement: Account<'info, Announcement>,

    /// AckRecord.announcement ⇔ announcement.key() の検証がないため、
    /// 偽のレコードで既読処理をすり抜け可能
    #[account(init, payer = user, space = 8 + 32 + 32 + 4 + 128)]
    pub record:       Account<'info, AckRecord>,

    #[account(mut)]
    pub author:       Signer<'info>,
    #[account(mut)]
    pub user:         Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ClearReads<'info> {
    /// AckRecord.user == user.key() は検証される
    #[account(mut, has_one = user)]
    pub record:       Account<'info, AckRecord>,

    /// Announcement.key() ⇔ record.announcement の検証がないため、
    /// 任意のレコードで別のお知らせの既読一覧をクリア可能
    #[account(mut)]
    pub announcement: Account<'info, Announcement>,

    pub user:         Signer<'info>,
}

#[program]
pub mod announcement_vuln {
    use super::*;

    /// お知らせを作成
    pub fn create_announcement(ctx: Context<CreateAnnouncement>, message: String) -> Result<()> {
        let a = &mut ctx.accounts.announcement;
        a.author        = ctx.accounts.author.key();
        a.message       = message;
        // acknowledgers は init 時点で空 Vec
        Ok(())
    }

    /// 既読をマーク（記録＋一覧追加）
    pub fn mark_read(ctx: Context<MarkRead>, note: String) -> Result<()> {
        let a  = &mut ctx.accounts.announcement;
        let r  = &mut ctx.accounts.record;

        // 脆弱性ポイント:
        // r.announcement = a.key(); の一致検証がない
        r.user          = ctx.accounts.user.key();
        r.announcement  = a.key();
        r.note          = note;

        // Vec::push で既読ユーザーを追加
        a.acknowledgers.push(r.user);
        Ok(())
    }

    /// 既読一覧をクリア （最後に追加されたユーザーを除去）
    pub fn clear_reads(ctx: Context<ClearReads>) -> Result<()> {
        let a = &mut ctx.accounts.announcement;

        // 本来必要:
        // require_keys_eq!(ctx.accounts.record.announcement, a.key(), ErrorCode::Mismatch);

        // Vec::pop で最後のユーザーを取り除く（分岐・ループなし）
        a.acknowledgers.pop();
        Ok(())
    }
}
