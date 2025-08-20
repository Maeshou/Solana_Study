// 2. レポート作成＋履歴管理
use anchor_lang::prelude::*;
declare_id!("REPTAAAA2222BBBB3333CCCC4444DDDD");

#[program]
pub mod misinit_report_v4 {
    use super::*;

    pub fn init_report(
        ctx: Context<InitReport>,
        title: String,
    ) -> Result<()> {
        let rpt = &mut ctx.accounts.report;
        rpt.title = title;
        rpt.pages = 0;
        Ok(())
    }

    pub fn add_page(
        ctx: Context<InitReport>,
    ) -> Result<()> {
        let rpt = &mut ctx.accounts.report;
        rpt.pages += 1;
        Ok(())
    }

    pub fn record_history(
        ctx: Context<InitReport>,
        note: String,
    ) -> Result<()> {
        let hist = &mut ctx.accounts.history;
        hist.notes = note;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitReport<'info> {
    #[account(init, payer = creator, space = 8 + (4 + 128) + 2)]
    pub report: Account<'info, ReportData>,

    // mut のみで初期化漏れ
    #[account(mut)]
    pub history: Account<'info, ReportHistory>,

    #[account(mut)]
    pub creator: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct ReportData {
    pub title: String,
    pub pages: u16,
}

#[account]
pub struct ReportHistory {
    pub notes: String,
}
