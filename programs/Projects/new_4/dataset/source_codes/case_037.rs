// 2. レポート＋履歴管理（Clockなし）
use anchor_lang::prelude::*;
declare_id!("REPTEEEEFFFF00001111222233334444");

#[program]
pub mod misinit_report_no_clock {
    use super::*;

    pub fn init_report(
        ctx: Context<InitReport>,
        title: String,
        summary: String,
    ) -> Result<()> {
        let rpt = &mut ctx.accounts.report;
        require!(summary.len() <= 256, ErrorCode2::SummaryTooLong);
        rpt.title = title;
        rpt.pages = 0;
        rpt.summary = summary;
        // 初期フラグ
        rpt.finalized = false;
        Ok(())
    }

    pub fn add_page(ctx: Context<InitReport>) -> Result<()> {
        let rpt = &mut ctx.accounts.report;
        rpt.pages = rpt.pages.checked_add(1).unwrap();
        Ok(())
    }

    pub fn finalize_report(ctx: Context<InitReport>) -> Result<()> {
        let rpt = &mut ctx.accounts.report;
        require!(!rpt.finalized, ErrorCode2::AlreadyFinalized);
        rpt.finalized = true;
        let hist = &mut ctx.accounts.history;
        if hist.notes.len() >= 10 { hist.notes.remove(0); }
        hist.notes.push(rpt.title.clone());
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitReport<'info> {
    #[account(init, payer = author, space = 8 + (4+64) + 2 + (4+256) + 1)]
    pub report: Account<'info, ReportData>,
    #[account(mut)] pub history: Account<'info, ReportHistory>,
    #[account(mut)] pub author: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct ReportData {
    pub title: String,
    pub pages: u16,
    pub summary: String,
    pub finalized: bool,
}

#[account]
pub struct ReportHistory { pub notes: Vec<String> }

#[error_code]
pub enum ErrorCode2 {
    #[msg("要約が長すぎます。255文字以内で入力してください。")]
    SummaryTooLong,
    #[msg("既に完了済みのレポートです。")]
    AlreadyFinalized,
}
