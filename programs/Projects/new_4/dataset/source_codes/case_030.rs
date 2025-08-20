// 2. レポート作成＋履歴管理
use anchor_lang::prelude::*;
declare_id!("REPTAAAA2222BBBB3333CCCC4444DDDD");

#[program]
pub mod misinit_report_v4 {
    use super::*;

    pub fn init_report(
        ctx: Context<InitReport>,
        title: String,
        summary: String,
    ) -> Result<()> {
        let rpt = &mut ctx.accounts.report;
        rpt.title = title;
        rpt.pages = 0;
        rpt.summary = summary;
        rpt.created_at = Clock::get()?.unix_timestamp;
        Ok(())
    }

    pub fn add_page(ctx: Context<InitReport>) -> Result<()> {
        let rpt = &mut ctx.accounts.report;
        rpt.pages = rpt.pages.checked_add(1).unwrap();
        rpt.updated_at = Some(Clock::get()?.unix_timestamp);
        Ok(())
    }

    pub fn record_history(ctx: Context<InitReport>, note: String) -> Result<()> {
        require!(note.len() < 256, ErrorCode::HistoryTooLong);
        let hist = &mut ctx.accounts.history;
        hist.notes.insert(0, note);
        if hist.notes.len() > 50 { hist.notes.pop(); }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitReport<'info> {
    #[account(init, payer = creator, space = 8 + (4+128) + 2 + (4+512) + 8 + 1)]
    pub report: Account<'info, ReportData>,
    #[account(mut)] pub history: Account<'info, ReportHistory>,
    #[account(mut)] pub creator: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct ReportData { pub title:String, pub pages:u16, pub summary:String, pub created_at:i64, pub updated_at:Option<i64> }
#[account]
pub struct ReportHistory { pub notes: Vec<String> }
#[error_code] pub enum ErrorCode2 { #[msg("履歴が長すぎます。255文字以内で入力してください。")] HistoryTooLong }
