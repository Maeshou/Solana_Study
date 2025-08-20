use anchor_lang::prelude::*;
declare_id!("BugFixVuln1111111111111111111111111111111");

/// バグチケット情報
#[account]
pub struct BugTicket {
    pub reporter: Pubkey,  // 報告者
    pub title:    String,  // チケットタイトル
    pub status:   String,  // "open", "in_progress", "fixed" など
}

/// 修正記録
#[account]
pub struct FixRecord {
    pub fixer:    Pubkey,  // 修正者
    pub ticket:   Pubkey,  // 本来は BugTicket.key() と一致すべき
    pub note:     String,  // 修正内容のメモ
}

#[derive(Accounts)]
pub struct CreateTicket<'info> {
    #[account(init, payer = reporter, space = 8 + 32 + 4 + 64 + 4 + 16)]
    pub ticket:   Account<'info, BugTicket>,
    #[account(mut)]
    pub reporter: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct RecordFix<'info> {
    /// BugTicket.reporter == reporter.key() は検証される
    #[account(mut, has_one = reporter)]
    pub ticket:   Account<'info, BugTicket>,

    /// FixRecord.ticket ⇔ ticket.key() の検証がないため、
    /// 偽のレコードで任意のチケットを修了扱いにできる
    #[account(init, payer = fixer, space = 8 + 32 + 32 + 4 + 128)]
    pub record:   Account<'info, FixRecord>,

    #[account(mut)]
    pub reporter: Signer<'info>,
    #[account(mut)]
    pub fixer:    Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ApplyFix<'info> {
    /// FixRecord.fixer == fixer.key() は検証される
    #[account(mut, has_one = fixer)]
    pub record:   Account<'info, FixRecord>,

    /// ticket.key() ⇔ record.ticket の検証がないため、
    /// 偽のレコードで別チケットのステータスを変えられる
    #[account(mut)]
    pub ticket:   Account<'info, BugTicket>,

    pub fixer:    Signer<'info>,
}

#[program]
pub mod bugfix_vuln {
    use super::*;

    /// チケットを作成
    pub fn create_ticket(ctx: Context<CreateTicket>, title: String) -> Result<()> {
        let t = &mut ctx.accounts.ticket;
        t.reporter = ctx.accounts.reporter.key();
        t.title    = title;
        t.status   = "open".to_string();
        Ok(())
    }

    /// 修正記録を残す
    pub fn record_fix(ctx: Context<RecordFix>, note: String) -> Result<()> {
        let r = &mut ctx.accounts.record;
        // 脆弱性ポイント:
        // r.ticket = ctx.accounts.ticket.key(); の一致検証がない
        r.fixer  = ctx.accounts.fixer.key();
        r.ticket = ctx.accounts.ticket.key();
        r.note   = note;
        Ok(())
    }

    /// 修正を適用してチケットを閉じる
    pub fn apply_fix(ctx: Context<ApplyFix>) -> Result<()> {
        let t = &mut ctx.accounts.ticket;
        // 本来必要:
        // require_keys_eq!(ctx.accounts.record.ticket, t.key(), ErrorCode::Mismatch);

        // ステータスを "fixed" に更新
        t.status = "fixed".to_uppercase();
        Ok(())
    }
}
